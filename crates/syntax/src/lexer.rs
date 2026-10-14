//! Turns Scribe source into tokens.
//!
//! Layout is part of the token stream: each line ends with a `Newline`, a
//! deeper indent opens an `Indent`, and a shallower one closes a `Dedent` for
//! each level it leaves. Blocks are four spaces deep; a tab is an error. Blank
//! lines and `note` lines are dropped, and the lines under an `about` header
//! become one `Prose` token.
//!
//! The rules match the reference grammar's (`grammar/scribe.lark` with
//! `tools/pre.py`), and `tools/compare_lexers.py` checks the two agree.

use crate::keywords::{WordKind, classify, is_keyword};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A word reserved everywhere.
    Hard,
    /// A word that is a keyword only inside its own phrase.
    Soft,
    Name,
    Int,
    Decimal,
    Version,
    Date,
    /// `0xFF`, with `_` allowed between digits.
    Hex,
    /// `0b1010`, with `_` allowed between digits.
    Binary,
    /// A number followed directly by a word that is not a keyword: `30 seconds`.
    Quantity,
    Str,
    /// The free text under an `about` header.
    Prose,
    Symbol,
    Newline,
    Indent,
    Dedent,
    /// A `...` line, which only the pilot corpora use as a placeholder.
    Elided,
}

impl TokenKind {
    /// The name `tools/compare_lexers.py` uses for this kind.
    pub fn oracle_name(self) -> &'static str {
        match self {
            TokenKind::Hard => "hard",
            TokenKind::Soft => "soft",
            TokenKind::Name => "name",
            TokenKind::Int => "int",
            TokenKind::Decimal => "decimal",
            TokenKind::Version => "version",
            TokenKind::Date => "date",
            TokenKind::Hex => "hex",
            TokenKind::Binary => "binary",
            TokenKind::Quantity => "quantity",
            TokenKind::Str => "string",
            TokenKind::Prose => "prose",
            TokenKind::Symbol => "symbol",
            TokenKind::Newline => "newline",
            TokenKind::Indent => "indent",
            TokenKind::Dedent => "dedent",
            TokenKind::Elided => "elided",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    /// 1-based line and column of the token's first character.
    pub line: usize,
    pub column: usize,
}

/// A lexing error, in the plain-English style of Section 13.2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for LexError {}

#[derive(Clone, Copy, Debug, Default)]
pub struct LexOptions {
    /// Accept `...` lines as `Elided` tokens (the pilot corpora's placeholder).
    pub elision: bool,
    /// Accept a fragment whose first line is indented, measuring indentation
    /// from that line (code quoted in documents).
    pub fragment: bool,
}

const INDENT_WIDTH: usize = 4;

const SYMBOLS_2: [&str; 6] = ["<=", ">=", "!=", "==", "&&", "||"];
const SYMBOLS_1: [char; 22] = [
    '+', '-', '*', '/', '=', '<', '>', '(', ')', '[', ']', ',', '!', '≤', '≥', '≠', '∧', '∨', '¬', '⇒', '∀',
    '∃',
];
const MEMBER: char = '∈';

pub fn lex(source: &str, options: LexOptions) -> Result<Vec<Token>, LexError> {
    let lines: Vec<&str> = source.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).collect();
    let mut out = Vec::new();
    let mut stack = vec![0usize];
    let mut base: Option<usize> = None;
    let mut i = 0;
    while i < lines.len() {
        let number = i + 1;
        let raw = lines[i].trim_end();
        let content = raw.trim_start_matches([' ', '\t']);
        if content.is_empty() || content == "note" || content.starts_with("note ") {
            i += 1;
            continue;
        }
        let spaces = indent_of(raw, number)?;
        let base_indent = *base.get_or_insert(if options.fragment { spaces } else { 0 });
        if spaces < base_indent {
            return Err(error(number, 1, "this line is indented less than the first line"));
        }
        let col = spaces - base_indent;
        if col % INDENT_WIDTH != 0 {
            return Err(error(number, spaces + 1, "indentation must be a multiple of four spaces"));
        }
        let top = *stack.last().unwrap();
        if col > top {
            if col != top + INDENT_WIDTH {
                return Err(error(number, spaces + 1, "this line is indented more than one level"));
            }
            stack.push(col);
            out.push(layout(TokenKind::Indent, number, spaces + 1));
        }
        while col < *stack.last().unwrap() {
            stack.pop();
            out.push(layout(TokenKind::Dedent, number, spaces + 1));
        }
        if col != *stack.last().unwrap() {
            return Err(error(
                number,
                spaces + 1,
                "this line does not line up with any block it could close",
            ));
        }

        if content == "about" {
            out.push(word_token("about", number, spaces + 1));
            let mut prose = Vec::new();
            let mut j = i + 1;
            while j < lines.len() {
                let next = lines[j].trim_end();
                if next.trim().is_empty() {
                    j += 1;
                    continue;
                }
                if indent_of(next, j + 1)?.saturating_sub(base_indent) <= col {
                    break;
                }
                prose.push(next.trim());
                j += 1;
            }
            out.push(Token {
                kind: TokenKind::Prose,
                text: prose.join(" "),
                line: number,
                column: spaces + 1,
            });
            out.push(layout(TokenKind::Newline, number, raw.chars().count() + 1));
            i = j;
            continue;
        }

        if options.elision && is_elision(content) {
            out.push(Token {
                kind: TokenKind::Elided,
                text: content.to_string(),
                line: number,
                column: spaces + 1,
            });
        } else {
            lex_line(content, number, spaces + 1, &mut out)?;
        }
        out.push(layout(TokenKind::Newline, number, raw.chars().count() + 1));
        i += 1;
    }
    let end = lines.len();
    for _ in 1..stack.len() {
        out.push(layout(TokenKind::Dedent, end, 1));
    }
    Ok(out)
}

fn error(line: usize, column: usize, message: &str) -> LexError {
    LexError { line, column, message: message.to_string() }
}

fn layout(kind: TokenKind, line: usize, column: usize) -> Token {
    Token { kind, text: String::new(), line, column }
}

/// The number of leading spaces; a tab among them is an error.
fn indent_of(line: &str, number: usize) -> Result<usize, LexError> {
    let mut spaces = 0;
    for (k, c) in line.chars().enumerate() {
        match c {
            ' ' => spaces += 1,
            '\t' => return Err(error(number, k + 1, "a tab is not allowed; indent with four spaces")),
            _ => break,
        }
    }
    Ok(spaces)
}

/// `...`, optionally with text and a closing `...`: the pilot's placeholder line.
fn is_elision(content: &str) -> bool {
    let Some(rest) = content.strip_prefix("...") else { return false };
    let middle = rest.strip_suffix("...").unwrap_or(rest);
    !middle.contains('.')
}

/// If `word` at `start` begins a two-word phrase the lexer reads as one token
/// (`for each`, `at entry`), the end of the phrase.
fn joined_word_phrase(chars: &[char], word: &str, word_end: usize) -> Option<usize> {
    let second = match word {
        "for" => "each",
        "at" => "entry",
        _ => return None,
    };
    let mut e = word_end;
    while chars.get(e) == Some(&' ') {
        e += 1;
    }
    if e == word_end {
        return None;
    }
    word_at(chars, e, second)
}

/// If the whole word `word` starts at `k`, the index just after it.
fn word_at(chars: &[char], k: usize, word: &str) -> Option<usize> {
    let end = k + word.chars().count();
    let matches = chars.get(k..end).is_some_and(|s| s.iter().copied().eq(word.chars()));
    let whole = !chars.get(end).is_some_and(|&c| is_word_char(c));
    (matches && whole).then_some(end)
}

fn word_token(word: &str, line: usize, column: usize) -> Token {
    let kind = match classify(word) {
        WordKind::Hard => TokenKind::Hard,
        WordKind::Soft => TokenKind::Soft,
        WordKind::Name => TokenKind::Name,
    };
    Token { kind, text: word.to_string(), line, column }
}

fn is_word_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn lex_line(content: &str, line: usize, first_column: usize, out: &mut Vec<Token>) -> Result<(), LexError> {
    let chars: Vec<char> = content.chars().collect();
    let mut k = 0;
    while k < chars.len() {
        let c = chars[k];
        let column = first_column + k;
        if c == ' ' {
            k += 1;
            continue;
        }
        if c == '\t' {
            return Err(error(line, column, "a tab is not allowed; use spaces"));
        }
        if c == '"' {
            let start = k;
            k += 1;
            loop {
                match chars.get(k) {
                    None => return Err(error(line, column, "this string is not closed")),
                    Some('\\') => k += 2,
                    Some('"') => {
                        k += 1;
                        break;
                    }
                    Some(_) => k += 1,
                }
            }
            let text: String = chars[start..k].iter().collect();
            out.push(Token { kind: TokenKind::Str, text, line, column });
            continue;
        }
        if c.is_ascii_digit() {
            let (kind, end) = number_at(&chars, k);
            let text: String = chars[k..end].iter().collect();
            out.push(Token { kind, text, line, column });
            k = end;
            continue;
        }
        if is_word_start(c) {
            let start = k;
            while k < chars.len() && is_word_char(chars[k]) {
                k += 1;
            }
            let word: String = chars[start..k].iter().collect();
            if let Some(end) = joined_word_phrase(&chars, &word, k) {
                let text: String = chars[start..end].iter().collect();
                out.push(Token { kind: TokenKind::Hard, text, line, column });
                k = end;
                continue;
            }
            out.push(word_token(&word, line, column));
            continue;
        }
        if c == ',' {
            // `, or else` and `, or` are read as one token (see JOINED in
            // tools/build_grammar.py); any other comma, including one that ends
            // the line, is a symbol
            let mut e = k + 1;
            while chars.get(e) == Some(&' ') {
                e += 1;
            }
            if let Some(after_or) = word_at(&chars, e, "or") {
                let mut f = after_or;
                while chars.get(f) == Some(&' ') {
                    f += 1;
                }
                let end = match word_at(&chars, f, "else") {
                    Some(after_else) if f > after_or => after_else,
                    _ => after_or,
                };
                let text: String = chars[k..end].iter().collect();
                out.push(Token { kind: TokenKind::Hard, text, line, column });
                k = end;
                continue;
            }
        }
        if c == '\'' && chars.get(k + 1) == Some(&'s') {
            out.push(Token { kind: TokenKind::Symbol, text: "'s".to_string(), line, column });
            k += 2;
            continue;
        }
        if let Some(two) = chars.get(k..k + 2).map(|s| s.iter().collect::<String>())
            && SYMBOLS_2.contains(&two.as_str())
        {
            out.push(Token { kind: TokenKind::Symbol, text: two, line, column });
            k += 2;
            continue;
        }
        if SYMBOLS_1.contains(&c) || c == MEMBER {
            out.push(Token { kind: TokenKind::Symbol, text: c.to_string(), line, column });
            k += 1;
            continue;
        }
        return Err(error(line, column, &format!("`{c}` is not part of Scribe")));
    }
    Ok(())
}

fn digits(chars: &[char], mut k: usize) -> usize {
    while k < chars.len() && chars[k].is_ascii_digit() {
        k += 1;
    }
    k
}

/// After the digits at `k`, a `.` and more digits.
fn fraction(chars: &[char], k: usize) -> Option<usize> {
    if chars.get(k) == Some(&'.') && chars.get(k + 1).is_some_and(|c| c.is_ascii_digit()) {
        Some(digits(chars, k + 1))
    } else {
        None
    }
}

/// After `0x` or `0b` at `k`, the end of the digits that `valid` accepts, with
/// single underscores between them; `None` if no digit follows the prefix.
fn radix_digits(chars: &[char], k: usize, valid: fn(char) -> bool) -> Option<usize> {
    let mut e = k + 2;
    if !chars.get(e).is_some_and(|&c| valid(c)) {
        return None;
    }
    e += 1;
    loop {
        match (chars.get(e), chars.get(e + 1)) {
            (Some(&c), _) if valid(c) => e += 1,
            (Some('_'), Some(&c)) if valid(c) => e += 2,
            _ => return Some(e),
        }
    }
}

/// The kind and end of the number starting at `k`, in the grammar's order of
/// priority: quantity, then hex, binary and date, then version, decimal, integer.
fn number_at(chars: &[char], k: usize) -> (TokenKind, usize) {
    if chars[k] == '0' {
        match chars.get(k + 1) {
            Some('x') => {
                if let Some(e) = radix_digits(chars, k, |c| c.is_ascii_hexdigit()) {
                    return (TokenKind::Hex, e);
                }
            }
            Some('b') => {
                if let Some(e) = radix_digits(chars, k, |c| c == '0' || c == '1') {
                    return (TokenKind::Binary, e);
                }
            }
            _ => {}
        }
    }
    let int_end = digits(chars, k);

    // Quantity: a number, spaces, then a word that is not a keyword.
    let num_end = fraction(chars, int_end).unwrap_or(int_end);
    let mut s = num_end;
    while chars.get(s) == Some(&' ') {
        s += 1;
    }
    if s > num_end && chars.get(s).is_some_and(|&c| is_word_start(c)) {
        let mut e = s;
        while e < chars.len() && is_word_char(chars[e]) {
            e += 1;
        }
        let word: String = chars[s..e].iter().collect();
        if !is_keyword(&word) {
            return (TokenKind::Quantity, e);
        }
    }

    // Date: four digits, two, two.
    if int_end - k == 4 && chars.get(int_end) == Some(&'-') {
        let m = digits(chars, int_end + 1);
        if m - (int_end + 1) == 2 && chars.get(m) == Some(&'-') {
            let d = digits(chars, m + 1);
            if d - (m + 1) == 2 {
                return (TokenKind::Date, d);
            }
        }
    }

    if let Some(minor) = fraction(chars, int_end) {
        if let Some(patch) = fraction(chars, minor) {
            return (TokenKind::Version, patch);
        }
        return (TokenKind::Decimal, minor);
    }
    (TokenKind::Int, int_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<(TokenKind, String)> {
        lex(src, LexOptions::default()).unwrap().into_iter().map(|t| (t.kind, t.text)).collect()
    }

    fn texts(src: &str) -> Vec<String> {
        kinds(src).into_iter().map(|(_, t)| t).collect()
    }

    #[test]
    fn layout_opens_and_closes_blocks() {
        let src = "to f x\n    given x is an Int\n    return x\nlet y be 1\n";
        let k: Vec<TokenKind> = kinds(src).into_iter().map(|(k, _)| k).collect();
        use TokenKind::*;
        assert_eq!(
            k,
            vec![
                Hard, Name, Name, Newline, Indent, Hard, Name, Hard, Hard, Name, Newline, Hard, Name,
                Newline, Dedent, Hard, Name, Hard, Int, Newline
            ]
        );
    }

    #[test]
    fn closes_every_block_at_the_end() {
        let src = "to f x\n    if x\n        return x\n";
        let dedents = kinds(src).iter().filter(|(k, _)| *k == TokenKind::Dedent).count();
        assert_eq!(dedents, 2);
    }

    #[test]
    fn drops_blank_and_note_lines() {
        assert_eq!(texts("note hello\n\nlet x be 1\n"), vec!["let", "x", "be", "1", ""]);
    }

    #[test]
    fn rejects_tabs_and_odd_indentation() {
        assert!(lex("to f x\n\tgiven x is an Int\n", LexOptions::default()).is_err());
        assert!(lex("to f x\n   given x is an Int\n", LexOptions::default()).is_err());
        assert!(lex("to f x\n        given x is an Int\n", LexOptions::default()).is_err());
    }

    #[test]
    fn numbers() {
        let k = kinds("let a be [1, 2.5, 3.0.0, 2026-10-14, 30 seconds, 4 of items]\n");
        let numbers: Vec<_> = k
            .into_iter()
            .filter(|(k, _)| {
                matches!(
                    k,
                    TokenKind::Int
                        | TokenKind::Decimal
                        | TokenKind::Version
                        | TokenKind::Date
                        | TokenKind::Quantity
                )
            })
            .collect();
        assert_eq!(
            numbers,
            vec![
                (TokenKind::Int, "1".into()),
                (TokenKind::Decimal, "2.5".into()),
                (TokenKind::Version, "3.0.0".into()),
                (TokenKind::Date, "2026-10-14".into()),
                (TokenKind::Quantity, "30 seconds".into()),
                (TokenKind::Int, "4".into()),
            ]
        );
    }

    #[test]
    fn hex_and_binary() {
        let k = kinds("let a be [0xFF, 0xFFFF_FFFF, 0b1010_0001, 0]\n");
        assert!(k.contains(&(TokenKind::Hex, "0xFF".into())));
        assert!(k.contains(&(TokenKind::Hex, "0xFFFF_FFFF".into())));
        assert!(k.contains(&(TokenKind::Binary, "0b1010_0001".into())));
        assert!(k.contains(&(TokenKind::Int, "0".into())));
    }

    #[test]
    fn strings_keep_escapes() {
        let k = kinds("let s be \"a \\\"quoted\\\" {x} word\"\n");
        assert_eq!(k[3], (TokenKind::Str, "\"a \\\"quoted\\\" {x} word\"".into()));
        assert!(lex("let s be \"open\n", LexOptions::default()).is_err());
    }

    #[test]
    fn symbols_longest_first() {
        assert_eq!(texts("return a <= b != c\n")[2..6], ["<=", "b", "!=", "c"]);
        assert_eq!(texts("return x ≤ y ∧ ¬ z\n")[1..7], ["x", "≤", "y", "∧", "¬", "z"]);
    }

    #[test]
    fn joined_phrases() {
        let k = kinds("let a be [x for each x in xs], or else nothing\n");
        assert!(k.contains(&(TokenKind::Hard, "for each".into())));
        assert!(k.contains(&(TokenKind::Hard, ", or else".into())));
        let k = kinds("server sends one of Done, or Failed\n");
        assert!(k.contains(&(TokenKind::Hard, ", or".into())));
        let k = kinds("ensure x = x at entry\n");
        assert!(k.contains(&(TokenKind::Hard, "at entry".into())));
        let k = kinds("let a be [1, order]\n");
        assert!(k.contains(&(TokenKind::Name, "order".into())));
        assert!(!k.iter().any(|(_, t)| t.starts_with(", or")));
    }

    #[test]
    fn possessives() {
        assert_eq!(texts("return user's manager's name\n")[1..6], ["user", "'s", "manager", "'s", "name"]);
    }

    #[test]
    fn prose_under_about() {
        let k = kinds(
            "component C version 1.0.0\n    about\n        Turns a cart\n        into an order.\n    needs\n",
        );
        assert!(k.contains(&(TokenKind::Prose, "Turns a cart into an order.".into())));
    }

    #[test]
    fn unknown_characters_are_errors() {
        let e = lex("let x be 1 ; 2\n", LexOptions::default()).unwrap_err();
        assert_eq!((e.line, e.column), (1, 12));
        assert!(e.message.contains('`'));
    }
}
