use std::io::{Stdin, Stdout, Write};

use crate::{lexer::Lexer, token::TokenKind};

pub struct REPL {
    stdin: Stdin,
    stdout: Stdout,
}

impl REPL {
    pub fn new(stdin: Stdin, stdout: Stdout) -> REPL {
        REPL { stdin, stdout } 
    }

    pub fn run(&mut self) {
        self.start();

        loop {
            write!(self.stdout, ">>> ").expect("Unable to write to stdout");
            self.stdout.flush().expect("Unable to flush stdout");

            let mut input = String::new();

            if let Err(error) = self.stdin.read_line(&mut input) {
                writeln!(self.stdout, "Error reading input: {}", error).expect("Unable to write to stdout");
            }

            let mut lexer = Lexer::new(input.as_str());

            loop {
                let token = lexer.next_token();

                if token.kind == TokenKind::EOF {
                    break;
                }

                writeln!(self.stdout, "{:?}", token).expect("Unable to write to stdout");
            }
        }
    }

    fn start(&mut self) {
        write!(self.stdout, "LeMonSeed v0.0.1-BETA REPL\n").expect("Unable to write to stdout");
        write!(self.stdout, "Feel free to type in commands\n").expect("Unable to write to stdout");
    }
}
