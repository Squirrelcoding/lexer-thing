use crate::{stmt::Stmt, interpreter::{Interpreter, err, self}, expr::Expr};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Func {
    pub instructions: Box<Stmt>,
    pub args: Vec<String>,
}

impl Func {
    pub fn new(instructions: Stmt, args: Vec<String>) -> Self {
        Self {
            instructions: Box::new(instructions),
            args,
        }
    }

    pub fn arity(&self) -> usize {
        self.args.len()
    }

    pub fn exec(&self, i: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, interpreter::Err> {
        todo!()
    }
}
