//! Turns tokens into a syntax tree, by recursive descent.
//!
//! The parser is written by hand, as the parsers of Rust, Go, Swift and
//! TypeScript are, so its error messages can say what was expected in the
//! reader's terms. It accepts exactly the language of the reference grammar
//! (`grammar/scribe.lark`), and `tools/compare_parsers.py` checks that the two
//! agree, tree for tree, on every test, spec block and corpus file.

use crate::ast::*;
use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

type Parse<T> = Result<T, ParseError>;

pub fn parse(tokens: &[Token]) -> Parse<File> {
    let mut p = Parser { tokens, pos: 0 };
    let mut items = Vec::new();
    while !p.at_end() {
        items.push(p.item()?);
    }
    Ok(File { items })
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

const CLAUSE_WORDS: [&str; 7] = ["can", "expect", "ensure", "changes", "uses", "example", "property"];

impl<'a> Parser<'a> {
    // ------------------------------------------------------------ tokens

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek_at(&self, n: usize) -> Option<&'a Token> {
        self.tokens.get(self.pos + n)
    }

    fn peek(&self) -> Option<&'a Token> {
        self.peek_at(0)
    }

    fn advance(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    fn is_kw_at(&self, n: usize, word: &str) -> bool {
        self.peek_at(n).is_some_and(|t| t.kind == TokenKind::Hard && t.text == word)
    }

    fn is_kw(&self, word: &str) -> bool {
        self.is_kw_at(0, word)
    }

    fn is_soft_at(&self, n: usize, word: &str) -> bool {
        self.peek_at(n).is_some_and(|t| t.kind == TokenKind::Soft && t.text == word)
    }

    fn is_sym_at(&self, n: usize, symbol: &str) -> bool {
        self.peek_at(n).is_some_and(|t| t.kind == TokenKind::Symbol && t.text == symbol)
    }

    fn is_sym(&self, symbol: &str) -> bool {
        self.is_sym_at(0, symbol)
    }

    fn is_kind_at(&self, n: usize, kind: TokenKind) -> bool {
        self.peek_at(n).is_some_and(|t| t.kind == kind)
    }

    fn is_kind(&self, kind: TokenKind) -> bool {
        self.is_kind_at(0, kind)
    }

    fn is_name_at(&self, n: usize) -> bool {
        self.peek_at(n).is_some_and(|t| matches!(t.kind, TokenKind::Name | TokenKind::Soft))
    }

    fn describe(t: Option<&Token>) -> String {
        match t {
            None => "the end of the file".into(),
            Some(t) => match t.kind {
                TokenKind::Newline => "the end of the line".into(),
                TokenKind::Indent => "an indented block".into(),
                TokenKind::Dedent => "the end of the block".into(),
                TokenKind::Elided => "`...`".into(),
                _ => format!("`{}`", t.text),
            },
        }
    }

    fn error_here(&self, message: String) -> ParseError {
        let (line, column) = match self.peek().or_else(|| self.tokens.last()) {
            Some(t) => (t.line, t.column),
            None => (1, 1),
        };
        ParseError { line, column, message }
    }

    fn expected(&self, what: &str) -> ParseError {
        self.error_here(format!("expected {what}, found {}", Self::describe(self.peek())))
    }

    fn expect_kw(&mut self, word: &str) -> Parse<()> {
        if self.is_kw(word) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected(&format!("`{word}`")))
        }
    }

    fn expect_soft(&mut self, word: &str) -> Parse<()> {
        if self.is_soft_at(0, word) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected(&format!("`{word}`")))
        }
    }

    fn expect_sym(&mut self, symbol: &str) -> Parse<()> {
        if self.is_sym(symbol) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected(&format!("`{symbol}`")))
        }
    }

    fn expect_kind(&mut self, kind: TokenKind, what: &str) -> Parse<()> {
        if self.is_kind(kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected(what))
        }
    }

    fn end_of_line(&mut self) -> Parse<()> {
        if self.is_kind(TokenKind::Newline) {
            self.advance();
            return Ok(());
        }
        Err(self.unexpected_in_line())
    }

    /// An error for a token where a line or a condition should have ended,
    /// naming the usual cause when there is one.
    fn unexpected_in_line(&self) -> ParseError {
        if self.is_sym("=") {
            return self
                .error_here("`=` binds or assigns, as a whole statement; to compare, write `==`".into());
        }
        let after_name = self.pos > 0 && self.is_name_at(0) && {
            let prev = &self.tokens[self.pos - 1];
            matches!(prev.kind, TokenKind::Name | TokenKind::Soft)
        };
        if after_name {
            return self.error_here(format!(
                "a call takes its arguments in brackets: `{}(...)`",
                self.tokens[self.pos - 1].text
            ));
        }
        self.expected("the end of the line")
    }

    fn name(&mut self) -> Parse<String> {
        if self.is_name_at(0) {
            Ok(self.advance().unwrap().text.clone())
        } else {
            Err(self.expected("a name"))
        }
    }

    fn names(&mut self) -> Parse<Vec<String>> {
        let mut out = vec![self.name()?];
        while self.is_sym(",") {
            self.advance();
            out.push(self.name()?);
        }
        Ok(out)
    }

    // ------------------------------------------------------------ items

    fn item(&mut self) -> Parse<Item> {
        if self.is_kw("function") {
            return Ok(Item::Function(self.function()?));
        }
        if self.is_kw("type") {
            return Ok(Item::Type(self.type_decl()?));
        }
        if self.is_kw("enum") {
            return Ok(Item::Enum(self.enum_decl()?));
        }
        if self.is_kw("module") {
            return Ok(Item::Module(self.module()?));
        }
        if self.is_kw("interface") {
            return Ok(Item::Interface(self.interface()?));
        }
        Ok(Item::Stmt(self.stmt()?))
    }

    fn function(&mut self) -> Parse<Function> {
        self.expect_kw("function")?;
        let name = self.name()?;
        self.expect_sym("(")?;
        let mut params = Vec::new();
        if !self.is_sym(")") {
            params.push(self.param()?);
            while self.is_sym(",") {
                self.advance();
                params.push(self.param()?);
            }
        }
        self.expect_sym(")")?;
        let result = if self.is_sym("->") {
            self.advance();
            Some(self.type_expr()?)
        } else {
            None
        };
        self.end_of_line()?;
        let mut contract = Vec::new();
        let mut body = Vec::new();
        if self.is_kind(TokenKind::Indent) {
            self.advance();
            while self
                .peek()
                .is_some_and(|t| t.kind == TokenKind::Hard && CLAUSE_WORDS.contains(&t.text.as_str()))
            {
                contract.push(self.clause()?);
            }
            while !self.is_kind(TokenKind::Dedent) && !self.at_end() {
                if self
                    .peek()
                    .is_some_and(|t| t.kind == TokenKind::Hard && CLAUSE_WORDS.contains(&t.text.as_str()))
                {
                    return Err(self.error_here(format!(
                        "`{}` belongs to the contract, which comes before the body",
                        self.peek().unwrap().text
                    )));
                }
                body.push(self.stmt()?);
            }
            self.expect_kind(TokenKind::Dedent, "the end of the block")?;
        }
        Ok(Function { name, params, result, contract, body })
    }

    fn param(&mut self) -> Parse<Param> {
        let name = self.name()?;
        if !self.is_sym(":") {
            return Err(self.expected("`:` and the parameter's type"));
        }
        self.advance();
        Ok(Param { name, ty: self.type_expr()? })
    }

    fn type_expr(&mut self) -> Parse<TypeExpr> {
        let mut modifiers = Vec::new();
        while self.is_kw("editable") || self.is_kw("consumed") {
            modifiers.push(self.advance().unwrap().text.clone());
        }
        let first = self.name()?;
        let (state, name) = if self.is_name_at(0) { (Some(first), self.name()?) } else { (None, first) };
        let mut args = Vec::new();
        if self.is_sym("[") {
            self.advance();
            args.push(self.type_expr()?);
            while self.is_sym(",") {
                self.advance();
                args.push(self.type_expr()?);
            }
            self.expect_sym("]")?;
        }
        let or_nothing = self.is_kw("or") && self.is_kw_at(1, "nothing");
        if or_nothing {
            self.pos += 2;
        }
        Ok(TypeExpr { modifiers, state, name, args, or_nothing })
    }

    fn range_from(&mut self, low: Expr) -> Parse<Range> {
        let word = self.advance().unwrap().text.clone();
        let high = self.sum()?;
        Ok(Range { low, word, high })
    }

    fn is_range_word(&self) -> bool {
        self.is_kw("until") || self.is_kw("through")
    }

    fn type_decl(&mut self) -> Parse<TypeDecl> {
        self.expect_kw("type")?;
        let name = self.name()?;
        let mut params = Vec::new();
        if self.is_sym("[") {
            self.advance();
            params = self.names()?;
            self.expect_sym("]")?;
        }
        let mut base = None;
        let mut range = None;
        if self.is_kw("is") {
            self.advance();
            base = Some(self.type_expr()?);
            if self.is_kw("in") {
                self.advance();
                let low = self.sum()?;
                if !self.is_range_word() {
                    return Err(self.expected("`until` or `through`"));
                }
                range = Some(self.range_from(low)?);
            }
        }
        self.end_of_line()?;
        let mut members = Vec::new();
        if self.is_kind(TokenKind::Indent) {
            self.advance();
            while !self.is_kind(TokenKind::Dedent) {
                members.push(self.member()?);
            }
            self.advance();
        }
        Ok(TypeDecl { name, params, base, range, members })
    }

    fn member(&mut self) -> Parse<Member> {
        if self.is_kw("model") {
            self.advance();
            let name = self.name()?;
            self.expect_sym("=")?;
            let value = self.expr()?;
            self.end_of_line()?;
            return Ok(Member::Model(name, value));
        }
        if self.is_kw("invariant") {
            self.advance();
            return Ok(Member::Invariant(self.line_condition()?));
        }
        if self.is_kw("states") {
            self.advance();
            let states = self.names()?;
            if states.len() < 2 {
                return Err(self.error_here("a type with states lists at least two".into()));
            }
            self.end_of_line()?;
            return Ok(Member::States(states));
        }
        let name = self.name()?;
        if !self.is_sym(":") {
            return Err(self.expected("`:` and the field's type"));
        }
        self.advance();
        let ty = self.type_expr()?;
        self.end_of_line()?;
        Ok(Member::Field(name, ty))
    }

    fn enum_decl(&mut self) -> Parse<EnumDecl> {
        self.expect_kw("enum")?;
        let name = self.name()?;
        self.end_of_line()?;
        self.expect_kind(TokenKind::Indent, "the enum's variants, indented")?;
        let mut variants = Vec::new();
        while !self.is_kind(TokenKind::Dedent) {
            let name = self.name()?;
            let mut params = Vec::new();
            if self.is_sym("(") {
                self.advance();
                params.push(self.param()?);
                while self.is_sym(",") {
                    self.advance();
                    params.push(self.param()?);
                }
                self.expect_sym(")")?;
            }
            self.end_of_line()?;
            variants.push(Variant { name, params });
        }
        self.advance();
        Ok(EnumDecl { name, variants })
    }

    fn interface(&mut self) -> Parse<Interface> {
        self.expect_kw("interface")?;
        let name = self.name()?;
        self.end_of_line()?;
        self.expect_kind(TokenKind::Indent, "the interface's functions, indented")?;
        let mut functions = Vec::new();
        while !self.is_kind(TokenKind::Dedent) {
            functions.push(self.function()?);
        }
        self.advance();
        Ok(Interface { name, functions })
    }

    fn module(&mut self) -> Parse<Module> {
        self.expect_kw("module")?;
        let name = self.name()?;
        self.end_of_line()?;
        self.expect_kind(TokenKind::Indent, "`requires` or `exports`, indented")?;
        let mut sections = Vec::new();
        while !self.is_kind(TokenKind::Dedent) {
            if self.is_soft_at(0, "requires") {
                self.advance();
                self.end_of_line()?;
                self.expect_kind(TokenKind::Indent, "the required modules, indented")?;
                let mut needs = Vec::new();
                while !self.is_kind(TokenKind::Dedent) {
                    let module = self.name()?;
                    self.expect_kw("as")?;
                    let alias = self.name()?;
                    self.end_of_line()?;
                    let mut uses = Vec::new();
                    if self.is_kind(TokenKind::Indent) {
                        self.advance();
                        self.expect_kw("uses")?;
                        uses = self.names()?;
                        self.end_of_line()?;
                        self.expect_kind(TokenKind::Dedent, "the end of the block")?;
                    }
                    needs.push(Need { module, alias, uses });
                }
                self.advance();
                sections.push(ModuleSection::Requires(needs));
            } else if self.is_soft_at(0, "exports") {
                self.advance();
                self.end_of_line()?;
                self.expect_kind(TokenKind::Indent, "the exported functions, indented")?;
                let mut functions = Vec::new();
                while !self.is_kind(TokenKind::Dedent) {
                    functions.push(self.function()?);
                }
                self.advance();
                sections.push(ModuleSection::Exports(functions));
            } else {
                return Err(self.expected("`requires` or `exports`"));
            }
        }
        self.advance();
        Ok(Module { name, sections })
    }

    // ------------------------------------------------------------ contracts

    fn clause(&mut self) -> Parse<Clause> {
        let word = self.advance().unwrap().text.clone();
        match word.as_str() {
            "can" => {
                self.expect_kw("fail")?;
                self.expect_kw("with")?;
                let failures = self.names()?;
                self.end_of_line()?;
                Ok(Clause::CanFail(failures))
            }
            "expect" => Ok(Clause::Expect(self.line_condition()?)),
            "ensure" => Ok(Clause::Ensure(self.ensure()?)),
            "changes" => {
                self.expect_kw("only")?;
                let mut places = vec![self.postfix()?];
                while self.is_sym(",") {
                    self.advance();
                    places.push(self.postfix()?);
                }
                self.end_of_line()?;
                Ok(Clause::Changes(places))
            }
            "uses" => {
                let names = self.names()?;
                self.end_of_line()?;
                Ok(Clause::Uses(names))
            }
            "example" => {
                let e = self.expr()?;
                self.end_of_line()?;
                Ok(Clause::Example(e))
            }
            _ => {
                if !self.at_quantifier() {
                    return Err(self.expected("`for all` or `there is`"));
                }
                let q = self.quantified()?;
                self.end_of_line()?;
                Ok(Clause::Property(q))
            }
        }
    }

    fn ensure(&mut self) -> Parse<Ensure> {
        if self.is_kind(TokenKind::Newline) {
            let (a, b) = self.if_then()?;
            return Ok(Ensure::IfThen(a, b));
        }
        if self.at_group() {
            return Ok(Ensure::Group(self.group()?));
        }
        let ensure = if self.is_kw("on") && self.is_kw_at(1, "success") {
            self.pos += 2;
            self.expect_sym(",")?;
            Ensure::OnSuccess(self.expr()?)
        } else if self.is_kw("on") && self.is_kw_at(1, "failure") {
            self.pos += 2;
            let mut failures = Vec::new();
            if !self.is_sym(",") {
                failures.push(self.name()?);
                while self.is_kw("or") {
                    self.advance();
                    failures.push(self.name()?);
                }
            }
            self.expect_sym(",")?;
            Ensure::OnFailure(failures, self.expr()?)
        } else if self.is_kw("if") {
            self.advance();
            let condition = self.logic()?;
            self.expect_sym(",")?;
            Ensure::If(condition, self.expr()?)
        } else {
            Ensure::Plain(self.expr()?)
        };
        self.end_of_line()?;
        Ok(ensure)
    }

    fn if_then(&mut self) -> Parse<(Expr, Expr)> {
        self.end_of_line()?;
        self.expect_kind(TokenKind::Indent, "an `if` line, indented")?;
        self.expect_kw("if")?;
        let a = self.condition()?;
        self.end_of_line()?;
        self.expect_kw("then")?;
        let b = self.condition()?;
        self.end_of_line()?;
        self.expect_kind(TokenKind::Dedent, "the end of the `if` / `then` block")?;
        Ok((a, b))
    }

    fn line_condition(&mut self) -> Parse<LineCondition> {
        if self.is_kind(TokenKind::Newline) {
            let (a, b) = self.if_then()?;
            return Ok(LineCondition::IfThen(a, b));
        }
        if self.at_group() {
            return Ok(LineCondition::Group(self.group()?));
        }
        let c = self.condition()?;
        self.end_of_line()?;
        Ok(LineCondition::Condition(c))
    }

    // ------------------------------------------------------------ groups

    fn at_group(&self) -> bool {
        ((self.is_kw("all") || self.is_kw("any")) && self.is_soft_at(1, "of"))
            || (self.is_kw("not") && self.is_kw_at(1, "any"))
    }

    fn group(&mut self) -> Parse<Group> {
        let kind = if self.is_kw("not") {
            self.pos += 2;
            "not any".to_string()
        } else {
            self.advance().unwrap().text.clone()
        };
        self.expect_soft("of")?;
        let mut items = Vec::new();
        if self.is_kind(TokenKind::Newline) {
            self.advance();
            self.expect_kind(TokenKind::Indent, "the group's conditions, indented")?;
            while !self.is_kind(TokenKind::Dedent) {
                if self.at_group() {
                    items.push(GroupItem::Group(self.group()?));
                    continue;
                }
                items.push(GroupItem::Condition(self.item_condition()?));
                if self.is_sym(",") {
                    self.advance();
                    self.end_of_line()?;
                    if self.is_kind(TokenKind::Dedent) {
                        return Err(self.error_here(
                            "a comma at the end of a line must be followed by another condition of the same group"
                                .into(),
                        ));
                    }
                } else {
                    self.end_of_line()?;
                }
            }
            self.advance();
        } else {
            items.push(GroupItem::Condition(self.inline_item()?));
            while self.is_sym(",") {
                self.advance();
                items.push(GroupItem::Condition(self.inline_item()?));
            }
            if items.len() < 2 {
                return Err(self.expected("`,` and another condition"));
            }
            self.end_of_line()?;
        }
        Ok(Group { kind, items })
    }

    fn item_condition(&mut self) -> Parse<Expr> {
        if self.at_quantifier() {
            return self.quantified();
        }
        self.inline_item()
    }

    fn inline_item(&mut self) -> Parse<Expr> {
        if self.at_not() {
            self.advance();
            return Ok(Expr::Not(Box::new(self.comparison()?)));
        }
        self.comparison()
    }

    // ------------------------------------------------------------ statements

    fn block(&mut self) -> Parse<Vec<Stmt>> {
        self.end_of_line()?;
        self.expect_kind(TokenKind::Indent, "an indented block")?;
        let mut stmts = Vec::new();
        while !self.is_kind(TokenKind::Dedent) && !self.at_end() {
            stmts.push(self.stmt()?);
        }
        self.expect_kind(TokenKind::Dedent, "the end of the block")?;
        Ok(stmts)
    }

    fn stmt(&mut self) -> Parse<Stmt> {
        if self.is_kind(TokenKind::Elided) {
            self.advance();
            self.end_of_line()?;
            return Ok(Stmt::Elided);
        }
        let word = match self.peek() {
            Some(t) if t.kind == TokenKind::Hard => t.text.as_str(),
            _ => "",
        };
        match word {
            "let" => {
                self.advance();
                let name = self.name()?;
                self.expect_sym("=")?;
                if self.at_group() {
                    return Ok(Stmt::Let { name, value: Value::Group(self.group()?), fallback: None });
                }
                let value = self.expr()?;
                let fallback = if self.is_kw("else") {
                    self.advance();
                    Some(self.expr()?)
                } else {
                    None
                };
                self.end_of_line()?;
                Ok(Stmt::Let { name, value: Value::Expr(value), fallback })
            }
            "var" => {
                self.advance();
                let name = self.name()?;
                self.expect_sym("=")?;
                let value = self.expr()?;
                self.end_of_line()?;
                Ok(Stmt::Var { name, value })
            }
            "return" => {
                self.advance();
                if self.is_kind(TokenKind::Newline) {
                    self.advance();
                    return Ok(Stmt::Return(None));
                }
                if self.at_group() {
                    return Ok(Stmt::Return(Some(Value::Group(self.group()?))));
                }
                let value = self.expr()?;
                self.end_of_line()?;
                Ok(Stmt::Return(Some(Value::Expr(value))))
            }
            "fail" => {
                self.advance();
                self.expect_kw("with")?;
                let value = self.expr()?;
                self.end_of_line()?;
                Ok(Stmt::Fail(value))
            }
            "do" => {
                self.advance();
                self.expect_kw("nothing")?;
                self.end_of_line()?;
                Ok(Stmt::DoNothing)
            }
            "if" => {
                self.advance();
                let mut branches = vec![self.if_body()?];
                let mut otherwise = None;
                while self.is_kw("else") {
                    self.advance();
                    if self.is_kw("if") {
                        self.advance();
                        branches.push(self.if_body()?);
                    } else {
                        otherwise = Some(self.block()?);
                        break;
                    }
                }
                Ok(Stmt::If { branches, otherwise })
            }
            "for" => {
                self.advance();
                let name = self.name()?;
                self.expect_kw("in")?;
                let first = self.sum()?;
                let iter = if self.is_range_word() {
                    ForIter::Range(self.range_from(first)?)
                } else {
                    ForIter::Expr(self.expr_from_sum(first)?)
                };
                let body = self.block()?;
                Ok(Stmt::For { name, iter, body })
            }
            "while" => {
                self.advance();
                let condition = if self.at_group() {
                    let g = self.group()?;
                    self.expect_kw("do")?;
                    Condition::Group(g)
                } else {
                    Condition::Expr(self.expr()?)
                };
                let body = self.block()?;
                Ok(Stmt::While { condition, body })
            }
            "invariant" => {
                self.advance();
                Ok(Stmt::Invariant(self.line_condition()?))
            }
            "assert" => {
                self.advance();
                Ok(Stmt::Assert(self.line_condition()?))
            }
            "decreases" => {
                self.advance();
                let e = self.expr()?;
                self.end_of_line()?;
                Ok(Stmt::Decreases(e))
            }
            "match" => {
                self.advance();
                let subject = self.expr()?;
                self.end_of_line()?;
                self.expect_kind(TokenKind::Indent, "the `case` lines, indented")?;
                let mut cases = Vec::new();
                while !self.is_kind(TokenKind::Dedent) {
                    self.expect_kw("case")?;
                    let variant = self.name()?;
                    let mut bindings = Vec::new();
                    if self.is_sym("(") {
                        self.advance();
                        bindings = self.names()?;
                        self.expect_sym(")")?;
                    }
                    let body = self.block()?;
                    cases.push(Case { variant, bindings, body });
                }
                self.advance();
                Ok(Stmt::Match { subject, cases })
            }
            "handle" => {
                self.advance();
                let body = self.block()?;
                let mut handlers = Vec::new();
                while self.is_kw("on") {
                    self.advance();
                    self.expect_kw("failure")?;
                    let mut failures = vec![self.name()?];
                    while self.is_kw("or") {
                        self.advance();
                        failures.push(self.name()?);
                    }
                    let binding = if self.is_kw("as") {
                        self.advance();
                        Some(self.name()?)
                    } else {
                        None
                    };
                    let body = self.block()?;
                    handlers.push(Handler { failures, binding, body });
                }
                if handlers.is_empty() {
                    return Err(self.expected("`on failure` and the failures it handles"));
                }
                Ok(Stmt::Handle { body, handlers })
            }
            "hold" => {
                self.advance();
                let name = self.name()?;
                self.expect_sym("=")?;
                let value = self.expr()?;
                let body = self.block()?;
                Ok(Stmt::Hold { name, value, body })
            }
            "together" => {
                self.advance();
                Ok(Stmt::Together(self.block()?))
            }
            "trust" => {
                self.advance();
                Ok(Stmt::Trust(self.block()?))
            }
            "break" | "continue" => {
                self.advance();
                self.end_of_line()?;
                Ok(if word == "break" { Stmt::Break } else { Stmt::Continue })
            }
            "try" => {
                self.advance();
                let e = self.postfix()?;
                self.end_of_line()?;
                Ok(Stmt::Try(e))
            }
            _ => {
                if !self.is_name_at(0)
                    && !self.peek().is_some_and(|t| {
                        matches!(
                            t.kind,
                            TokenKind::Int
                                | TokenKind::Decimal
                                | TokenKind::Str
                                | TokenKind::Quantity
                                | TokenKind::Hex
                                | TokenKind::Binary
                        ) || (t.kind == TokenKind::Symbol && (t.text == "(" || t.text == "["))
                            || (t.kind == TokenKind::Hard
                                && (t.text == "result" || t.text == "nothing" || t.text == "old"))
                    })
                {
                    return Err(self.expected("a statement"));
                }
                let target = self.postfix()?;
                if self.is_sym("=") {
                    self.advance();
                    let value = self.expr()?;
                    self.end_of_line()?;
                    return Ok(Stmt::Assign { target, value });
                }
                if !self.is_kind(TokenKind::Newline) {
                    return Err(self.error_here(format!(
                        "a statement is an assignment or a call; found {} after a value",
                        Self::describe(self.peek())
                    )));
                }
                self.advance();
                Ok(Stmt::Expr(target))
            }
        }
    }

    fn if_body(&mut self) -> Parse<(Condition, Vec<Stmt>)> {
        if self.at_group() {
            let g = self.group()?;
            self.expect_kw("then")?;
            let body = self.block()?;
            return Ok((Condition::Group(g), body));
        }
        let e = self.expr()?;
        let body = self.block()?;
        Ok((Condition::Expr(e), body))
    }

    // ------------------------------------------------------------ conditions

    fn at_quantifier(&self) -> bool {
        (self.is_kw("for") && self.is_kw_at(1, "all"))
            || self.is_sym("∀")
            || (self.is_kw("there") && self.is_kw_at(1, "is"))
            || self.is_sym("∃")
    }

    fn at_not(&self) -> bool {
        self.is_kw("not") || self.is_sym("¬") || self.is_sym("!")
    }

    pub(crate) fn expr(&mut self) -> Parse<Expr> {
        self.condition()
    }

    fn condition(&mut self) -> Parse<Expr> {
        if self.at_quantifier() {
            return self.quantified();
        }
        if self.at_not() {
            self.advance();
            let c = self.comparison()?;
            if self.at_and() || self.at_or() {
                return Err(self.error_here(
                    "`not` negates a whole condition, not one link of a chain; use a condition group".into(),
                ));
            }
            return Ok(Expr::Not(Box::new(c)));
        }
        self.logic()
    }

    fn quantified(&mut self) -> Parse<Expr> {
        if (self.is_kw("there") && self.is_kw_at(1, "is")) || self.is_sym("∃") {
            self.pos += if self.is_sym("∃") { 1 } else { 2 };
            let var = self.name()?;
            self.expect_kw("in")?;
            let domain = self.sum()?;
            self.expect_kw("where")?;
            let condition = self.condition()?;
            return Ok(Expr::Exists { var, domain: Box::new(domain), condition: Box::new(condition) });
        }
        self.pos += if self.is_sym("∀") { 1 } else { 2 };
        let var = self.name()?;
        let domain = if self.is_kw("in") {
            self.advance();
            let first = self.sum()?;
            if self.is_range_word() { Domain::Range(self.range_from(first)?) } else { Domain::Expr(first) }
        } else if self.is_sym(":") {
            self.advance();
            Domain::Type(self.type_expr()?)
        } else {
            return Err(self.expected("`in` or `:` and a type"));
        };
        self.expect_sym(",")?;
        let condition = self.condition()?;
        Ok(Expr::ForAll { var, domain: Box::new(domain), condition: Box::new(condition) })
    }

    fn at_and(&self) -> bool {
        self.is_kw("and") || self.is_sym("∧") || self.is_sym("&&")
    }

    fn at_or(&self) -> bool {
        self.is_kw("or") || self.is_sym("∨") || self.is_sym("||")
    }

    fn logic(&mut self) -> Parse<Expr> {
        let first = self.comparison()?;
        self.logic_from(first)
    }

    fn logic_from(&mut self, first: Expr) -> Parse<Expr> {
        let mixed =
            || "`and` and `or` cannot be mixed on one line; use a condition group (`all of`, `any of`)";
        if self.at_and() {
            let mut parts = vec![first];
            while self.at_and() {
                self.advance();
                parts.push(self.comparison()?);
            }
            if self.at_or() {
                return Err(self.error_here(mixed().into()));
            }
            return Ok(Expr::And(parts));
        }
        if self.at_or() {
            let mut parts = vec![first];
            while self.at_or() {
                self.advance();
                parts.push(self.comparison()?);
            }
            if self.at_and() {
                return Err(self.error_here(mixed().into()));
            }
            return Ok(Expr::Or(parts));
        }
        Ok(first)
    }

    /// An expression whose first sum has already been read (after `for x in`).
    fn expr_from_sum(&mut self, first: Expr) -> Parse<Expr> {
        let c = self.comparison_from(first)?;
        self.logic_from(c)
    }

    fn comparison(&mut self) -> Parse<Expr> {
        let first = self.sum()?;
        self.comparison_from(first)
    }

    fn comparison_op(&mut self) -> Option<String> {
        let t = self.peek()?;
        let op = match (t.kind, t.text.as_str()) {
            (TokenKind::Symbol, "==") => "==",
            (TokenKind::Symbol, "!=" | "≠") => "!=",
            (TokenKind::Symbol, "<") => "<",
            (TokenKind::Symbol, ">") => ">",
            (TokenKind::Symbol, "<=" | "≤") => "<=",
            (TokenKind::Symbol, ">=" | "≥") => ">=",
            (TokenKind::Symbol, "∈") => "in",
            (TokenKind::Hard, "in") => "in",
            (TokenKind::Hard, "is") => {
                if self.is_kw_at(1, "not") {
                    self.pos += 2;
                    return Some("is-not".into());
                }
                "is"
            }
            (TokenKind::Hard, "not") if self.is_kw_at(1, "in") => {
                self.pos += 2;
                return Some("not-in".into());
            }
            _ => return None,
        };
        self.advance();
        Some(op.into())
    }

    fn comparison_from(&mut self, first: Expr) -> Parse<Expr> {
        let mut rest = Vec::new();
        while let Some(op) = self.comparison_op() {
            rest.push((op, self.sum()?));
        }
        if rest.is_empty() { Ok(first) } else { Ok(Expr::Compare(Box::new(first), rest)) }
    }

    // ------------------------------------------------------------ expressions

    fn sum(&mut self) -> Parse<Expr> {
        let mut left = self.product()?;
        while self.is_sym("+") || self.is_sym("-") {
            let op = self.advance().unwrap().text.clone();
            let right = self.product()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn product(&mut self) -> Parse<Expr> {
        let mut left = self.unary()?;
        while self.is_sym("*") || self.is_sym("/") || self.is_sym("%") {
            let op = self.advance().unwrap().text.clone();
            let right = self.unary()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Parse<Expr> {
        if self.is_sym("-") {
            self.advance();
            return Ok(Expr::Neg(Box::new(self.unary()?)));
        }
        if self.is_kw("try") {
            self.advance();
            return Ok(Expr::Try(Box::new(self.unary()?)));
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Parse<Expr> {
        let mut e = self.atom()?;
        loop {
            if self.is_sym(".") {
                self.advance();
                e = Expr::Field(Box::new(e), self.name()?);
            } else if self.is_sym("(") {
                self.advance();
                let mut args = Vec::new();
                if !self.is_sym(")") {
                    args.push(self.arg()?);
                    while self.is_sym(",") {
                        self.advance();
                        args.push(self.arg()?);
                    }
                }
                self.expect_sym(")")?;
                e = Expr::Call(Box::new(e), args);
            } else if self.is_sym("[") {
                self.advance();
                let index = self.sum()?;
                self.expect_sym("]")?;
                e = Expr::Index(Box::new(e), Box::new(index));
            } else {
                return Ok(e);
            }
        }
    }

    fn arg(&mut self) -> Parse<Arg> {
        if self.is_name_at(0) && self.is_sym_at(1, ":") {
            let name = self.name()?;
            self.advance();
            return Ok(Arg { name: Some(name), value: self.expr()? });
        }
        Ok(Arg { name: None, value: self.expr()? })
    }

    fn atom(&mut self) -> Parse<Expr> {
        let Some(t) = self.peek() else {
            return Err(self.expected("a value"));
        };
        match t.kind {
            TokenKind::Int
            | TokenKind::Decimal
            | TokenKind::Str
            | TokenKind::Quantity
            | TokenKind::Hex
            | TokenKind::Binary => {
                self.advance();
                Ok(Expr::Literal(t.text.clone()))
            }
            TokenKind::Name | TokenKind::Soft => {
                self.advance();
                Ok(Expr::Name(t.text.clone()))
            }
            TokenKind::Hard if t.text == "result" => {
                self.advance();
                Ok(Expr::Result)
            }
            TokenKind::Hard if t.text == "nothing" => {
                self.advance();
                Ok(Expr::Nothing)
            }
            TokenKind::Hard if t.text == "old" => {
                self.advance();
                self.expect_sym("(")?;
                let e = self.expr()?;
                self.expect_sym(")")?;
                Ok(Expr::Old(Box::new(e)))
            }
            TokenKind::Symbol if t.text == "(" => {
                self.advance();
                let e = self.sum()?;
                if !self.is_sym(")") {
                    return Err(self.error_here(format!(
                        "brackets group arithmetic only; expected `)`, found {}",
                        Self::describe(self.peek())
                    )));
                }
                self.advance();
                Ok(e)
            }
            TokenKind::Symbol if t.text == "[" => {
                self.advance();
                if self.is_sym("]") {
                    self.advance();
                    return Ok(Expr::List(Vec::new()));
                }
                let first = self.comparison()?;
                if self.is_kw("for") {
                    self.advance();
                    let var = self.name()?;
                    self.expect_kw("in")?;
                    let source = self.sum()?;
                    let filter = if self.is_kw("where") {
                        self.advance();
                        Some(Box::new(self.condition()?))
                    } else {
                        None
                    };
                    self.expect_sym("]")?;
                    return Ok(Expr::Comprehension {
                        element: Box::new(first),
                        var,
                        source: Box::new(source),
                        filter,
                    });
                }
                let mut items = vec![first];
                while self.is_sym(",") {
                    self.advance();
                    items.push(self.comparison()?);
                }
                self.expect_sym("]")?;
                Ok(Expr::List(items))
            }
            _ => Err(self.expected("a value")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::to_sexpr;
    use crate::lexer::{LexOptions, lex};

    fn tree(src: &str) -> String {
        to_sexpr(&parse(&lex(src, LexOptions::default()).unwrap()).unwrap())
    }

    fn error(src: &str) -> ParseError {
        parse(&lex(src, LexOptions::default()).unwrap()).unwrap_err()
    }

    #[test]
    fn functions_and_contracts() {
        let t = tree(
            "function withdraw(amount: Money, account: editable Account) -> Receipt\n    can fail with Low\n    expect amount > 0\n    ensure on success, account.balance == old(account.balance) - amount\n    account.balance = account.balance - amount\n    return Receipt(amount: amount)\n",
        );
        assert!(t.contains("(param account (type editable Account))"));
        assert!(t.contains("(can-fail Low)"));
        assert!(t.contains(
            "(on-success (compare (field account balance) == (- (old (field account balance)) amount)))"
        ));
        assert!(t.contains("(return (call Receipt (named amount amount)))"));
    }

    #[test]
    fn arithmetic_follows_school_precedence() {
        assert!(tree("let x = a + b * c - d\n").contains("(let x (- (+ a (* b c)) d))"));
    }

    #[test]
    fn typestate_and_or_nothing() {
        let t = tree("function f(c: consumed Open Connection) -> User or nothing\n    return nothing\n");
        assert!(t.contains("(type consumed state=Open Connection)"));
        assert!(t.contains("(type User ?)"));
    }

    #[test]
    fn mixing_and_with_or_is_an_error() {
        let e = error("function f(a: Bool, b: Bool, c: Bool) -> Bool\n    return a and b or c\n");
        assert!(e.message.contains("condition group"), "{}", e.message);
        assert_eq!(e.line, 2);
    }

    #[test]
    fn single_equals_is_not_a_comparison() {
        let e = error("function f(a: Int, b: Int)\n    if a = b\n        return\n");
        assert!(e.message.contains("write `==`"), "{}", e.message);
    }

    #[test]
    fn a_call_needs_brackets() {
        let e = error("function f(items: List[Int]) -> Int\n    return count of items\n");
        assert!(e.message.contains("`count(...)`"), "{}", e.message);
    }

    #[test]
    fn brackets_never_group_logic() {
        let e = error("let x = (a and b)\n");
        assert!(e.message.contains("brackets group arithmetic only"), "{}", e.message);
    }

    #[test]
    fn a_bare_expression_is_not_a_statement() {
        let e = error("function f(x: Int)\n    x + 1\n");
        assert!(e.message.contains("assignment or a call"), "{}", e.message);
    }

    #[test]
    fn clauses_come_before_the_body() {
        let e = error("function f(x: Int)\n    let y = x\n    expect x > 0\n");
        assert!(e.message.contains("contract"), "{}", e.message);
    }
}
