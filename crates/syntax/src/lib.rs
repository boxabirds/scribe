//! Scribe's front end: the lexer now, the parser and formatter next
//! (roadmap phase 2).

pub mod keywords;
pub mod lexer;

pub use lexer::{LexError, LexOptions, Token, TokenKind, lex};
