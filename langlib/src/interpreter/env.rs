use std::{cell::RefCell, collections::HashMap};

use crate::expr::Expr;

use super::err::RuntimeErr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Env {
    vals: HashMap<String, Expr>,
    parent: Option<Box<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
            parent: None,
        }
    }

    /// Tries to get a variable from the environment.
    pub fn get(&self, k: &str) -> Result<Expr, RuntimeErr> {
        match self.vals.get(k) {
            Some(v) => Ok(v.to_owned()),

            // If there's a parent environment then attempt to get the variable from that.
            None => match &self.parent {
                Some(parent) => parent.borrow().get(k),
                None => Err(RuntimeErr::UndefinedVar(k.to_owned())),
            },
        }
    }

    /// Defines a new variable and stores it in the environment.
    pub fn define(&mut self, k: String, v: Expr) {
        self.vals.insert(k, v);
    }

    /// Assigns a value to a variable.
    pub fn assign(&mut self, k: String, v: Expr) -> Result<(), RuntimeErr> {
        match self.vals.get(&k) {
            Some(_) => {
                self.vals.insert(k, v);
                Ok(())
            }
            None => match &self.parent {
                Some(parent) => RefCell::borrow_mut(parent).assign(k, v),
                None => todo!(),
            },
        }
    }

    /// Sets the parent of the environment.
    pub fn set_parent(&mut self, parent: &Env) {
        self.parent = Some(Box::new(RefCell::new(parent.clone())));
    }

    /// Gets the parent of the environment.
    pub fn get_parent(self) -> Option<Env> {
        self.parent
            .as_ref()
            .map(|parent| parent.as_ref().to_owned().into_inner())
    }
}
