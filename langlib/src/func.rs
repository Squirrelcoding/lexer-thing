use crate::{
    expr::Expr,
    interpreter::{self, env::Env, Interpreter},
    stmt::Stmt,
};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Func {
    pub instructions: Box<Stmt>,
    pub args: Vec<String>,
    pub closure: Option<Env>,
}

impl Func {
    pub fn new(instructions: Stmt, args: Vec<String>) -> Self {
        Self {
            instructions: Box::new(instructions),
            args,
            closure: None,
        }
    }

    pub fn arg_len(&self) -> usize {
        self.args.len()
    }

    pub fn exec(
        self,
        interpreter: &mut Interpreter,
        args: Vec<Expr>,
    ) -> Result<Expr, interpreter::Err> {
        if self.closure.is_none() {
            panic!("Function has no closure. If you see this message than the code of the interpreter is fucked up.");
        }
        
        let save = interpreter.env.clone().into_inner();
        
        let mut new_env = Env::default();

        new_env.set_parent(self.closure.unwrap());
        
        // Bring all the variables into scope
        (0..args.len()).for_each(|i| {
            new_env.define(self.args[i].clone(), args[i].clone());
        });

        interpreter.env.replace(new_env);
        
        let return_val = match interpreter.execute_stmt(&self.instructions) {
            Ok(_) => Expr::Null,
            Err(err) => match err {
                interpreter::Err::ReturnStmt(expr) => expr,
                err => return Err(err),
            },
        };

        interpreter.env.replace(save);

        Ok(return_val)
    }

    /// Sets the closure of this function.
    pub fn set_closure(&mut self, closure: Env) {
        self.closure = Some(closure);
    }
}
