use thiserror::Error;

use crate::expr::Expr;

use super::op::{BinOp, UnOp};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Op(BinOp),
    UnOp(UnOp),
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
    pub fn try_into_op(self) -> Result<BinOp, TokenError> {
        if let Token::Op(op) = self {
            Ok(op)
        } else {
            Err(TokenError::FailedConversion)
        }
    }

    pub fn try_into_int(self) -> Result<i32, TokenError> {
        if let Token::Int(int) = self {
            Ok(int)
        } else {
            Err(TokenError::FailedConversion)
        }
    }

    pub fn try_into_un_op(self) -> Result<UnOp, TokenError> {
        if let Token::UnOp(unop) = self {
            Ok(unop)
        } else {
            Err(TokenError::FailedConversion)
        }
    }

    pub fn try_into_ident(self) -> Result<String, TokenError> {
        if let Token::Ident(ident) = self {
            Ok(ident)
        } else {
            Err(TokenError::FailedConversion)
        }
    }

    pub fn try_into_keyword(self) -> Result<String, TokenError> {
        if let Token::Keyword(keyword) = self {
            Ok(keyword)
        } else {
            Err(TokenError::FailedConversion)
        }
    }

    pub fn into_expr(self) -> Result<Expr, TokenError> {
        match self {
            Token::Int(int) => Ok(Expr::Num(int)),
            Token::String(string) => Ok(Expr::Str(string)),
            Token::Keyword(val) => match val.as_str() {
                "true" => Ok(Expr::Bool(true)),
                "false" => Ok(Expr::Bool(false)),
                _ => todo!(),
            },
            _ => Err(TokenError::FailedConversion),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum TokenError {
    #[error("An invalid token conversion was attemped.")]
    FailedConversion,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Keyword {

}

#[cfg(test)]

mod token_tests {
    use crate::{
        expr::Expr,
        lexer::op::{BinOp, UnOp},
    };

    use super::Token;

    #[test]
    fn successful_conversions() {
        let a = Token::Op(BinOp::Add);
        let b = Token::Int(32);
        let c = Token::UnOp(UnOp::Bang);
        let d = Token::Ident("hello".to_owned());
        let e = Token::Keyword("keyword".to_owned());

        assert!(a.try_into_op().is_ok());
        assert!(b.try_into_int().is_ok());
        assert!(c.try_into_un_op().is_ok());
        assert!(d.try_into_ident().is_ok());
        assert!(e.try_into_keyword().is_ok());
    }

    #[test]
    fn failed_conversions() {
        let a = Token::Op(BinOp::Add);
        let b = Token::Int(32);
        let c = Token::UnOp(UnOp::Bang);
        let d = Token::Ident("hello".to_owned());
        let e = Token::Keyword("keyword".to_owned());

        assert!(e.try_into_op().is_err());
        assert!(d.try_into_int().is_err());
        assert!(c.try_into_int().is_err());
        assert!(b.try_into_ident().is_err());
        assert!(a.try_into_keyword().is_err());
    }

    #[test]
    fn successful_into_exprs() {
        let string = Token::String("This is a cool string.".to_owned()).into_expr();
        let num = Token::Int(23).into_expr();
        let true_bool = Token::Keyword("true".to_owned()).into_expr();
        let false_bool = Token::Keyword("false".to_owned()).into_expr();

        assert!(string.is_ok());
        assert!(num.is_ok());
        assert!(true_bool.is_ok());
        assert!(false_bool.is_ok());

        let string = string.unwrap();
        let num = num.unwrap();
        let true_bool = true_bool.unwrap();
        let false_bool = false_bool.unwrap();

        assert_eq!(string, Expr::Str("This is a cool string.".to_owned()));
        assert_eq!(num, Expr::Num(23));
        assert_eq!(true_bool, Expr::Bool(true));
        assert_eq!(false_bool, Expr::Bool(false));
    }
}
