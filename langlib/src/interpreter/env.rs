use std::collections::HashMap;

use crate::expr::Expr;

use super::err::RuntimeErr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Env {
    vals: HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }

    pub fn get(&self, k: &str) -> Result<&Expr, RuntimeErr> {
        match self.vals.get(k) {
            Some(v) => Ok(v),
            None => Err(RuntimeErr::UndefinedVar(k.to_owned())),
        }
    }

    pub fn define(&mut self, k: String, v: Expr) -> Result<(), RuntimeErr> {
        if self.vals.get(&k).is_some() {
            return Err(RuntimeErr::VarRedefine(k));
        }

        self.vals.insert(k, v);

        Ok(())
    }
}
