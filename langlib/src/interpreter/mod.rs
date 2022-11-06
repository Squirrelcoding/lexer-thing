mod env;
mod err;

use err::RuntimeErr;
use std::io;

use crate::{
    expr::Expr,
    lexer::{Lexer, LexerError},
    parser::{error::ParserError, Parser},
    stmt::Stmt,
};

use self::env::Env;

#[derive(Debug)]
pub struct Interpreter {
    instructions: Vec<Stmt>,
    env: Env,
}

impl Interpreter {
    pub fn new(instructions: Vec<Stmt>) -> Self {
        Self {
            instructions,
            env: Env::new(),
        }
    }

    /// Interprets the instructions, and consumes itself.
    pub fn interpret(mut self) -> Result<(), Err> {
        for stmt in self.instructions {
            match stmt {
                Stmt::Declaration(declaration) => {
                    if let Err(err) = self.env.define(declaration.ident, declaration.val) {
                        return Err(Err::RuntimeErr(err));
                    }
                }
                Stmt::Print(expr) => {
                    if let Expr::Var(ident) = &expr {
                        let expr = self.env.get(ident)?;
                        println!("{expr}");
                    } else {
                        println!("{}", expr.eval()?);
                    }
                }
                Stmt::ExprStatement(expr) => {
                    println!("{expr}");
                    
                }
            }
        }

        Ok(())
    }

    pub fn repl() -> Result<(), Err> {
        loop {
            let mut input_string = String::new();

            if input_string == "EXIT" {
                break;
            }
            io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string

            let result = Parser::new(Lexer::new(&input_string).tokenize()?)
                .expr()?
                .eval()?;

            println!("{result}");
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
    #[error("A runtime error has occured.")]
    RuntimeErr(#[from] RuntimeErr),
}
