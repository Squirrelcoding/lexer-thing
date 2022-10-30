use crate::stmt::Stmt;

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
}
