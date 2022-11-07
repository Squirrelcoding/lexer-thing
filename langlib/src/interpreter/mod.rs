mod env;
mod err;

use err::RuntimeErr;
use std::{cell::RefCell, io};

use crate::{
    expr::{BinExpr, Expr},
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

    /// Visits an expression and executes it.
    fn visit_expr(&self, expr: Expr) -> Result<Expr, Err> {
        match expr {
            Expr::Var(var) => match self.env.get(&var) {
                Ok(val) => {
                    Ok(val.clone())
                },
                Err(err) => Err(Err::RuntimeErr(err)),
            },
            Expr::Bin(bin_expr) => {


                let lhs = self.visit_expr(bin_expr.lhs.as_ref().clone())?;
                
                let rhs = self.visit_expr(bin_expr.rhs.as_ref().clone())?;


                match Expr::eval(&Expr::Bin(BinExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    op: bin_expr.op,
                })) {
                    Ok(result) => Ok(result),
                    Err(err) => Err(Err::ParserError(err)),
                }
            },
            Expr::Unary(op, expr) => match self.visit_expr(Expr::Unary(op, expr)) {
                Ok(val) => Ok(val),
                Err(err) => Err(err),
            },
            _ => Ok(expr),
        }
    }

    /// Interprets the instructions
    pub fn interpret(&mut self) -> Result<(), Err> {
        for stmt in self.instructions.iter() {
            match stmt {
                Stmt::Declaration(declaration) => {
                    if let Err(err) = self
                        .env
                        .define(declaration.ident.clone(), declaration.val.clone())
                    {
                        return Err(Err::RuntimeErr(err));
                    }
                }

                Stmt::Print(exprr) => {

                    let result = self.visit_expr(exprr.clone())?;

                    println!("{}", result);
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
