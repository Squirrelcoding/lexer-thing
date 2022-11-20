pub mod env;
pub mod err;

use err::RuntimeErr;
use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::{self, Read},
    path::Path,
};

use crate::{
    expr::{BinExpr, Expr},
    func::Func,
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
            env: RefCell::new(Env::default()),
        })
    }

    pub fn new(instructions: Vec<Stmt>) -> Self {
        Self {
            instructions,
            env: RefCell::new(Env::default()),
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
                self.visit_expr(expr)?;
            }

            Stmt::Block(stmts) => {
                let prev = self.env.to_owned().into_inner();
                
                let mut new_env = Env::default();
                new_env.set_parent(prev);
                
                self.env.replace(new_env);
                
                // The return statement that can be mutated
                let mut return_stmt: Expr = Expr::Null;

                for block_stmt in stmts {

                    // If there is an error there's a chance that it's of the variant "ReturnStmt"
                    match self.execute_stmt(block_stmt) {
                        Ok(_) => continue,
                        Err(err) => match err {
                            Err::ReturnStmt(stmt) => match stmt {
                                Stmt::Return(expr) => {

                                    // Set the return statement to the expr and break;
                                    return_stmt = expr;
                                    break;
                                },
                                _ => unreachable!(),
                            },
                            err => return Err(err),
                        },
                    };
                }
                
                // Clean up
                let parent = self.env.to_owned().into_inner().get_parent().unwrap();
                self.env.replace(parent);

                // Return the return statement
                if return_stmt != Expr::Null {
                    self.execute_stmt(&Stmt::Return(return_stmt))?
                }
            },
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
            },
            Stmt::Return(expr) => {
                return Err(Err::ReturnStmt(Stmt::Return(self.visit_expr(expr)?)));
            }
        }

        Ok(())
    }

    /// Visits an expression and executes it.
    fn visit_expr(&self, expr: &Expr) -> Result<Expr, Err> {
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
                let func = self.visit_expr(callee)?;

                let func = match func {
                    Expr::Func(func) => func,
                    _ => {
                        return Err(Err::RuntimeErr(RuntimeErr::UnexpectedType(
                            err::LexerThingType::Func,
                        )))
                    }
                };

                let args: Vec<Expr> = args
                    .iter()
                    .map(|expr| self.visit_expr(expr))
                    .try_collect()?;

                if func.arity() != args.len() {
                    return Err(Err::RuntimeErr(RuntimeErr::BadArgLength(
                        func.arity(),
                        args.len(),
                    )));
                }

                self.exec_func(func, args)
            }

            _ => Ok(expr.clone()),
        }
    }

    pub fn exec_func(&self, func: Func, args: Vec<Expr>) -> Result<Expr, Err> {

        let prev = self.env.to_owned().into_inner();

        let mut new_env = Env::default();
        new_env.set_parent(prev);

        self.env.replace(new_env);

        // Bring all the variables into scope.
        (0..args.len()).for_each(|i| {
            self.env
                .borrow_mut()
                .define(func.args[i].clone(), args[i].to_owned());
        });

        let return_val = match self.execute_stmt(&func.instructions) {
            Ok(_) => Ok(Expr::Null),
            Err(err) => match err {
                Err::ReturnStmt(stmt) => match stmt {
                    Stmt::Return(expr) => Ok(expr),
                    _ => unreachable!(),
                },
                err => Err(err),
            },
        };

        let parent = self.env.to_owned().into_inner().get_parent().unwrap();

        self.env.replace(parent);

        return_val
    }

    // Helper functions for other structs
    pub fn define_var(&self, k: String, v: Expr) {
        self.env.borrow_mut().define(k, v);
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

    #[error("Not really an error.")]
    ReturnStmt(Stmt),
}
