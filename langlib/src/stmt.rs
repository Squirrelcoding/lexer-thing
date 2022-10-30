use crate::lexer::token::Token;

use super::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    Assignment(Assignment),
    Print(String),
    ExprStatement(Expr),
}

impl Stmt {
    pub fn execute(&self) -> Option<Expr> {
        match self {
            Stmt::Assignment(assignment) => todo!(),
            Stmt::Print(str) => {
                println!("{str}");
                None
            }
            Stmt::ExprStatement(expr) => Some(expr.to_owned()),
        }
    }

    pub fn from_tokens(tokens: &[Token]) -> Result<Self, StmtErr> {
        match tokens {
            [Token::Keyword(keyword)] => match keyword.as_str() {
                "let" => {}
                _ => {}
            },
            _ => {}
        }

        todo!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum StmtErr {
    #[error("A failed conversion occured")]
    FailedConversion,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Assignment {
    pub ident: String,
    pub val: Expr,
}
