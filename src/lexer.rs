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
        self.skip_whitespace();

        println!("next_token: {}", self.ch);

        let token = match self.ch {
            '=' => Token::new(TokenKind::Assign, self.ch),
            ';' => Token::new(TokenKind::Semicolon, self.ch),
            '(' => Token::new(TokenKind::LeftParenthesis, self.ch),
            ')' => Token::new(TokenKind::RightParenthesis, self.ch),
            ',' => Token::new(TokenKind::Comma, self.ch),
            '+' => Token::new(TokenKind::Plus, self.ch),
            '{' => Token::new(TokenKind::LeftBrace, self.ch),
            '}' => Token::new(TokenKind::RightBrace, self.ch),
            '\0' => Token::new(TokenKind::EOF, ""),
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = TokenKind::lookup_identifier(&literal);
                    Token::new(kind, literal)
                } else if Lexer::is_digit(self.ch) {
                    let kind = TokenKind::Int;
                    let literal = self.read_number();
                    Token::new(kind, literal)
                } else {
                    Token::new(TokenKind::Illegal, self.ch)
                }
            }
        };

        println!("created token {:?}", token);
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

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }

        identifier
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let mut number = String::from("");

        while Lexer::is_digit(self.ch) {
            number.push(self.ch);
            self.read_char();
        }

        number
    }

    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_next_token_inline() {
        let input: &str = "=+(){},5;";
        let expected_tokens: Vec<Token> = vec![
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (idx, expected_token) in expected_tokens.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                expected_token.kind, token.kind,
                "tests[{idx}] - token kind wrong. expected={} | got={}",
                expected_token.kind, token.kind
            );
            assert_eq!(
                expected_token.literal, token.literal,
                "tests[{idx}] - token literal wrong. expected={} | got={}",
                expected_token.literal, token.literal
            );
        }
    }

    #[test]
    fn test_next_token_multiline() {
        let input: &str = r#"
        const five = 5;
        const ten = 10;

        const add = function(x, y) {
            x + y;
        }

        const result = add(five, ten);
        "#;
        let expected_tokens: Vec<Token> = vec![
            Token::new(TokenKind::Constant, "const"),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Constant, "const"),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Constant, "const"),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Function, "function"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Constant, "const"),
            Token::new(TokenKind::Identifier, "result"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (idx, expected_token) in expected_tokens.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                expected_token.kind, token.kind,
                "tests[{idx}] - token kind wrong. expected={} | got={}",
                expected_token.kind, token.kind
            );
            assert_eq!(
                expected_token.literal, token.literal,
                "tests[{idx}] - token literal wrong. expected={} | got={}",
                expected_token.literal, token.literal
            );
        }
    }
}
