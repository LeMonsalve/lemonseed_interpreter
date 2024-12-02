use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenKind::Equal, "==")
                } else {
                    Token::new(TokenKind::Assign, self.ch)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenKind::NotEqual, "!=")
                } else {
                    Token::new(TokenKind::Bang, self.ch)
                }
            }
            ';' => Token::new(TokenKind::Semicolon, self.ch),
            '(' => Token::new(TokenKind::LeftParenthesis, self.ch),
            ')' => Token::new(TokenKind::RightParenthesis, self.ch),
            ',' => Token::new(TokenKind::Comma, self.ch),
            '+' => Token::new(TokenKind::Plus, self.ch),
            '-' => Token::new(TokenKind::Minus, self.ch),
            '*' => Token::new(TokenKind::Asterisk, self.ch),
            '/' => Token::new(TokenKind::Slash, self.ch),
            '<' => Token::new(TokenKind::LessThan, self.ch),
            '>' => Token::new(TokenKind::GreaterThan, self.ch),
            '{' => Token::new(TokenKind::LeftBrace, self.ch),
            '}' => Token::new(TokenKind::RightBrace, self.ch),
            '[' => Token::new(TokenKind::LeftBracket, self.ch),
            ']' => Token::new(TokenKind::RightBracket, self.ch),
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

        self.read_char();

        token
    }

    fn peek_char(&self) -> char {
        return if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        };
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
    fn test_next_token_multiline() {
        let input: &str = r#"
        const five = 5;
        const ten = 10;

        const add = function(x, y) {
            x + y;
        }

        const result = add(five, ten);

        var a = 1;
        var b = 2;
        var c = a + b - (a * b) / (a + b);
        var d = a < b > c;

        if (a == b) {
            return true;
        } else {
            return false;
        }

        if (a != b) {
            return false;
        } else {
            return true;
        }

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        if (20 > 30) {
            return false;
        } else {
            return true;
        }
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
            Token::new(TokenKind::Variable, "var"),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "1"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Variable, "var"),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "2"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Variable, "var"),
            Token::new(TokenKind::Identifier, "c"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::Minus, "-"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::Asterisk, "*"),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::Slash, "/"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Variable, "var"),
            Token::new(TokenKind::Identifier, "d"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::LessThan, "<"),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::GreaterThan, ">"),
            Token::new(TokenKind::Identifier, "c"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::If, "if"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::Equal, "=="),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::True, "true"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Else, "else"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::False, "false"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::If, "if"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Identifier, "a"),
            Token::new(TokenKind::NotEqual, "!="),
            Token::new(TokenKind::Identifier, "b"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::False, "false"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Else, "else"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::True, "true"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::If, "if"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::LessThan, "<"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::True, "true"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Else, "else"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::False, "false"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::If, "if"),
            Token::new(TokenKind::LeftParenthesis, "("),
            Token::new(TokenKind::Int, "20"),
            Token::new(TokenKind::GreaterThan, ">"),
            Token::new(TokenKind::Int, "30"),
            Token::new(TokenKind::RightParenthesis, ")"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::False, "false"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::Else, "else"),
            Token::new(TokenKind::LeftBrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::True, "true"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::RightBrace, "}"),
            Token::new(TokenKind::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (idx, expected_token) in expected_tokens.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token, expected_token,
                "Test[{idx}] Token mismatch. Expected: {:?}, Got: {:?}",
                expected_token, token
            );
        }
    }
}
