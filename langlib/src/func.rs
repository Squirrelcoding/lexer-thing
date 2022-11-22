use crate::{
    expr::Expr,
    interpreter::{self, env::Env, Interpreter},
    stmt::Stmt,
};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Func {
    pub instructions: Box<Stmt>,
    pub args: Vec<String>,
    closure: Env,
}

impl Func {
    pub fn new(instructions: Stmt, args: Vec<String>) -> Self {
        Self {
            instructions: Box::new(instructions),
            args,
            closure: Env::default(),
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
        let prev = self.closure;

        let mut new_env = Env::default();
        new_env.set_parent(prev);

        interpreter.env.replace(new_env);

        // Bring all the variables into scope
        (0..args.len()).for_each(|i| {
            interpreter.define_var(self.args[i].clone(), args[i].clone());
        });

        // Get the instructions for the function
        let stmts = match self.instructions.as_ref() {
            Stmt::Block(stmts) => stmts,
            _ => todo!(),
        };

        // Keep executing until we encounter a return statement
        let return_val = match stmts
            .iter()
            .find_map(|stmt| match interpreter.execute_stmt(stmt) {
                Ok(_) => None,
                Err(err) => match err {
                    interpreter::Err::ReturnStmt(expr) => Some(Ok(expr)),
                    err => Some(Err(err)),
                },
            }) {
            Some(return_val) => Ok(return_val?),
            None => Ok(Expr::Null),
        };

        // Clean up the environment
        (0..args.len()).for_each(|i| {
            interpreter.env.borrow_mut().drop(&self.args[i]);
        });

        let parent = interpreter
            .env
            .to_owned()
            .into_inner()
            .parent
            .unwrap()
            .into_inner();
        interpreter.env.replace(parent);

        return_val
    }

    /// Sets the closure of this function.
    pub fn set_closure(&mut self, closure: Env) {
        self.closure = closure;
    }
}
