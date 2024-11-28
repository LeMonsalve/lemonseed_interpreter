use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParenthesis,
    RightParenthesis,

    LeftBrace,
    RightBrace,

    Function,
    Var
}

impl Token {
    pub fn new<S: Into<String>>(kind: TokenKind, literal: S) -> Token {
        Token { kind, literal: literal.into() }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::LeftParenthesis => write!(f, "("),
            TokenKind::RightParenthesis => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Var => write!(f, "Var"),
        }
    }
}