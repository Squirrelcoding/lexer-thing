use super::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Assignment {
    pub ident: String,
    pub val: Expr,
}
