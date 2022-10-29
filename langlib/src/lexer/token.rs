use thiserror::Error;

use crate::expr::Expr;

use super::op::Op;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Op(Op),
    AssignmentSign,
    EqSign,
    Int(i32),
    Semi,
    String(String),
    LeftBracket,
    RightBracket,
    Ident(String),
    Keyword(String),
}

impl Token {
    pub fn op(self) -> Result<Op, TokenError> {
        if let Token::Op(op) = self {
            Ok(op)
        } else {
            Err(TokenError::InvalidConversion)
        }
    }

    pub fn try_into_int(self) -> Result<i32, TokenError> {
        if let Token::Int(int) = self {
            Ok(int)
        } else {
            Err(TokenError::InvalidConversion)
        }
    }

    pub fn try_into_ident(self) -> Result<String, TokenError> {
        if let Token::Ident(ident) = self {
            Ok(ident)
        } else {
            Err(TokenError::InvalidConversion)
        }
    }

    pub fn try_into_keyword(self) -> Result<String, TokenError> {
        if let Token::Keyword(keyword) = self {
            Ok(keyword)
        } else {
            Err(TokenError::InvalidConversion)
        }
    }

    pub fn into_expr(self) -> Result<Expr, TokenError> {
        match self {
            Token::Int(int) => Ok(Expr::Num(int)),
            Token::String(string) => Ok(Expr::Str(string)),
            _ => Err(TokenError::InvalidConversion),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum TokenError {
    #[error("An invalid token conversion was attemped.")]
    InvalidConversion,
}