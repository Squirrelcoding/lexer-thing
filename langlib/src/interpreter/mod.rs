mod env;
mod err;

use err::RuntimeErr;
use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::{self, Read},
    path::Path,
};

use crate::{
    expr::{BinExpr, Expr},
    lexer::{err::LexerError, Lexer},
    parser::{err::ParserError, Parser},
    stmt::Stmt,
};

use self::env::Env;

#[derive(Debug)]
pub struct Interpreter {
    instructions: Vec<Stmt>,
    env: RefCell<Env>,
}

impl Interpreter {
    pub fn from_file(path: &Path) -> Result<Self, Err> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let mut source = String::new();

        file.read_to_string(&mut source)?;

        let stmts = Parser::new(Lexer::new(&source).tokenize()?).get_statements()?;

        Ok(Self {
            instructions: stmts,
            env: RefCell::new(Env::new()),
        })
    }

    pub fn new(instructions: Vec<Stmt>) -> Self {
        Self {
            instructions,
            env: RefCell::new(Env::new()),
        }
    }

    /// Visits an expression and executes it.
    fn  visit_expr(&self, expr: &Expr) -> Result<Expr, Err> {
        match expr {
            Expr::Var(var) => match self.env.borrow().get(var) {
                Ok(val) => Ok(val),
                Err(err) => Err(Err::RuntimeErr(err)),
            },
            Expr::Bin(bin_expr) => {
                let lhs = self.visit_expr(&bin_expr.lhs)?;

                let rhs = self.visit_expr(&bin_expr.rhs)?;

                match Expr::eval(&Expr::Bin(BinExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    op: bin_expr.op.clone(),
                })) {
                    Ok(result) => Ok(result),
                    Err(err) => Err(Err::ParserError(err)),
                }
            }
            Expr::Unary(op, expr) => match Expr::eval(&Expr::Unary(op.clone(), expr.clone())) {
                Ok(val) => Ok(val),
                Err(err) => Err(Err::ParserError(err)),
            },

            Expr::Funcall(callee, args) => {

                println!("{callee:?}");

                let args: Vec<Expr> = args.iter().map(|expr| self.visit_expr(expr)).try_collect()?;

                println!("ARGSSSSSSSSSSS: {args:?}");

                todo!()
            }

            _ => Ok(expr.clone()),
        }
    }

    /// Interprets the code
    pub fn interpret(&self) -> Result<(), Err> {
        for stmt in &self.instructions {
            self.execute_stmt(stmt)?;
        }

        Ok(())
    }

    /// Interprets the instructions
    pub fn execute_stmt(&self, stmt: &Stmt) -> Result<(), Err> {
        match stmt {
            Stmt::Declaration(declaration) => {
                let expr = self.visit_expr(&declaration.val)?;

                self.env
                    .borrow_mut()
                    .define(declaration.ident.to_owned(), expr);
            }

            Stmt::Print(exprr) => {
                let result = self.visit_expr(exprr)?.eval()?;

                println!("{result}");
            }

            Stmt::Expr(expr) => {
                println!("{}", self.visit_expr(&expr)?);
            }

            //TODO!!
            Stmt::Block(stmts) => {
                let prev = self.env.to_owned().into_inner();

                let mut new_env = Env::new();
                new_env.set_parent(prev);

                self.env.replace(new_env);

                for block_stmt in stmts {
                    self.execute_stmt(block_stmt)?;
                }

                let parent = self.env.to_owned().into_inner().get_parent().unwrap();

                self.env.replace(parent);
            }
            Stmt::If(expr, block, else_block) => {
                let result = match self.visit_expr(expr)? {
                    Expr::Bool(b) => b,
                    expr => return Err(Err::RuntimeErr(RuntimeErr::InvalidExpr(expr))),
                };

                if result {
                    self.execute_stmt(block)?;
                } else if let Some(else_block) = else_block {
                    self.execute_stmt(else_block)?;
                }
            }
            Stmt::While(condition, block) => {
                while self.visit_expr(condition)?.try_into()? {
                    self.execute_stmt(block)?;
                }
            }
            Stmt::Assignment(declaration) => {
                let expr = self.visit_expr(&declaration.val)?;

                self.env
                    .borrow_mut()
                    .assign(declaration.ident.to_owned(), expr)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Err {
    #[error("An error occurred during parsing.")]
    ParserError(#[from] ParserError),
    #[error("An error occurred during lexing.")]
    LexerError(#[from] LexerError),
    #[error("A runtime error has occured.")]
    RuntimeErr(#[from] RuntimeErr),

    #[error("An IO error occured while attempting to read the file.")]
    IOError(#[from] io::Error),
}
