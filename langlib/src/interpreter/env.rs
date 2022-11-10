use std::collections::HashMap;

use crate::expr::Expr;

use super::err::RuntimeErr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Env {
    vals: HashMap<String, Expr>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
            parent: None,
        }
    }

    /// Tries to get a variable from the environment.
    pub fn get(&self, k: &str) -> Result<&Expr, RuntimeErr> {
        match self.vals.get(k) {
            Some(v) => Ok(v),

            // If there's a parent environment then attempt to get the variable from that.
            None => match self.parent.as_ref() {
                Some(parent) => parent.get(k),
                None => Err(RuntimeErr::UndefinedVar(k.to_owned())),
            },
        }
    }

    /// Defines a new variable and stores it in the environment.
    pub fn define(&mut self, k: String, v: Expr) -> Result<(), RuntimeErr> {
        if self.parent.is_some() {
            self.parent
                .as_mut()
                .unwrap()
                .define(k.to_owned(), v.to_owned())?;
        }

        self.vals.insert(k, v);

        Ok(())
    }

    /// Sets the parent of the environment.
    pub fn set_parent(&mut self, parent: &Env) {
        self.parent = Some(Box::new(parent.clone()));
    }
}
