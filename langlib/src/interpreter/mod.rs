use std::io;

use crate::{
    lexer::{Lexer, LexerError},
    parser::{error::ParserError, Parser},
    stmt::Stmt,
};

#[derive(Debug)]
pub struct Interpreter {
    instructions: Vec<Stmt>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Stmt>) -> Self {
        Self { instructions }
    }

    pub fn interpret(&self) {
        self.instructions.iter().for_each(|stmt| {
            stmt.execute();
        });
    }

    pub fn repl() -> Result<(), Err> {
        loop {
            let mut input_string = String::new();
            io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string

            if input_string == ".exit" {
                break;
            }

            let result = Parser::new(Lexer::new(&input_string).tokenize()?)
                .expr()?
                .eval()?;

            println!("{result:?}");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Err {
    #[error("An error occurred during parsing.")]
    ParserError(#[from] ParserError),
    #[error("An error occurred during lexing.")]
    LexerError(#[from] LexerError),
}
