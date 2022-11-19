use crate::stmt::Stmt;
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
}
