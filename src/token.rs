pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

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