use crate::parser::error::ParserError;

use super::lexer::op::Op;

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum Expr {
    Num(i32),
    Str(String),
    Bool(bool),
    Bin(BinExpr),
}

impl Expr {
    pub fn eval(&self) -> Result<Expr, ParserError> {
        if let Expr::Bin(expr) = self {
            return expr.eval();
        }
        Ok(self.clone())
    }
}

impl TryInto<i32> for Expr {
    type Error = ParserError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Expr::Num(num) => Ok(num),
            _ => Err(ParserError::ExprError(ExprError::FailedConversion)),
        }
    }
}

impl TryInto<bool> for Expr {
    type Error = ParserError;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Expr::Bool(bool) => Ok(bool),
            _ => Err(ParserError::ExprError(ExprError::FailedConversion)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: Op,
}

impl BinExpr {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>, op: Op) -> Self {
        Self { lhs, rhs, op }
    }

    pub fn eval(&self) -> Result<Expr, ParserError> {
        match (self.lhs.as_ref(), self.rhs.as_ref()) {
            (Expr::Num(a), Expr::Num(b)) => Ok(Expr::Bool(a == b)),
            (Expr::Str(a), Expr::Str(b)) => Ok(Expr::Bool(a == b)),
            (Expr::Bool(a), Expr::Bool(b)) => Ok(Expr::Bool(a == b)),
            (Expr::Bin(a), Expr::Bin(b)) => Ok(Expr::Bool(a.eval()? == b.eval()?)),
            _ => Err(ParserError::ExprError(ExprError::FailedBinEvaluation)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ExprError {
    #[error("A failed conversion occured.")]
    FailedConversion,

    #[error("The parser failed to evaluate a binary expression")]
    FailedBinEvaluation,
}
