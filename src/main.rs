use std::io;

use repl::REPL;

pub mod token;
pub mod lexer;
pub mod repl;
pub mod ast;

fn main() {
    let mut repl = REPL::new(io::stdin(), io::stdout());
    repl.run();
}
