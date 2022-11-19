use crate::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum RuntimeErr {
    #[error("Variable \"{0}\" already exists.")]
    VarRedefine(String),
    #[error("Variable \"{0}\" does not exist.")]
    UndefinedVar(String),
    #[error("An invalid expression was found")]
    InvalidExpr(Expr),
    #[error("An unexpected type was found. Expected type '{0:?}'")]
    UnexpectedType(LexerThingType),
    #[error("An unexpected number of arguments was supplied. Expected {0} arguments, found {1}.")]
    BadArgLength(usize, usize),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LexerThingType {
    Int,
    Str,
    Bool,
    Null,
    Ident,
    Func,
}
