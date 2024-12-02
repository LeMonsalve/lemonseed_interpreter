use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Illegal,
    EOF,

    Identifier,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

    Comma,
    Semicolon,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Function,
    Variable,
    Constant,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn new<S: Into<String>>(kind: TokenKind, literal: S) -> Token {
        Token {
            kind,
            literal: literal.into(),
        }
    }
}

impl TokenKind {
    pub fn lookup_identifier<S: AsRef<str>>(literal: &S) -> TokenKind {
        match literal.as_ref() {
            "function" => TokenKind::Function,
            "var" => TokenKind::Variable,
            "const" => TokenKind::Constant,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::LeftParenthesis => write!(f, "("),
            TokenKind::RightParenthesis => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Variable => write!(f, "Variable"),
            TokenKind::Constant => write!(f, "Constant"),
            TokenKind::True =>  write!(f, "True"),
            TokenKind::False =>  write!(f, "False"),
            TokenKind::If =>  write!(f, "If"),
            TokenKind::Else =>  write!(f, "Else"),
            TokenKind::Return =>  write!(f, "Return"),
        }
    }
}
