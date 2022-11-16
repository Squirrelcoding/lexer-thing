use crate::{expr::Expr, stmt::Stmt, interpreter::{Interpreter, Err}};

type NativeFunc = fn() -> Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Func {
    ident: String,
    args: Vec<Expr>,
    instructions: Option<Vec<Stmt>>,
    code: Option<NativeFunc>,
    is_native: bool
}

impl Func {
    // pub fn new(ident: String, args: Vec<Expr>, instructions: Vec<Stmt>, is_native: bool) -> Self {

    //     Self {
    //         ident,
    //         args,
    //         Some(instructions),
    //         is_native
    //     }
    // }

    pub fn exec(&self, i: &mut Interpreter) -> Result<(), Err>{

        for instruction in &self.instructions {
            i.execute_stmt(instruction)?;
        }

        Ok(())
    }

    pub fn arity(&self) -> usize {
        self.args.len()
    }


}