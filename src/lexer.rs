use crate::token::{Token, TokenKind};

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };

        lexer.read_char();

        lexer
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token::new(TokenKind::Assign, self.ch),
            ';' => Token::new(TokenKind::Semicolon, self.ch),
            '(' => Token::new(TokenKind::LeftParenthesis, self.ch),
            ')' => Token::new(TokenKind::RightParenthesis, self.ch),
            ',' => Token::new(TokenKind::Comma, self.ch),
            '+' => Token::new(TokenKind::Plus, self.ch),
            '{' => Token::new(TokenKind::LeftBrace, self.ch),
            '}' => Token::new(TokenKind::RightBrace, self.ch),
            '\0' => Token::new(TokenKind::Eof, ""),
            _ => Token::new(TokenKind::Illegal, self.ch),
        };

        self.read_char();

        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenKind};
    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";

        let expected_tokens: Vec<Token> = vec![
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (idx, expected_token) in expected_tokens.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                expected_token.kind, token.kind,
                "tests[{idx}] - token kind wrong. expected={}, got={}",
                expected_token.kind, token.kind
            );
            assert_eq!(
                expected_token.literal, token.literal,
                "tests[{idx}] - token literal wrong. expected={}, got={}",
                expected_token.literal, token.literal
            );
        }
    }
}