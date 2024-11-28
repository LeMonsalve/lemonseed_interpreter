struct Token {
    kind: TokenKind,
    value: String,
}

enum TokenKind {
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