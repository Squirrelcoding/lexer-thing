use crate::{
    expr::Expr,
    lexer::{op::BinOp, token::Token},
};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse an arithmetic expression
    pub fn num_expr(&mut self) -> Result<Expr, ParserError> {
        let mut x: i32 = self.term()?.try_into()?;

        while !self.is_at_end() {

 let op = match self.matches(&[Token::Op(BinOp::Add), Token::Op(BinOp::Sub)]) {
                Some(op) => op,
                None => break,
            }
            .try_into_op()?;

            let other_term: i32 = self.term()?.try_into()?;
            

            match op {
                BinOp::Add => x += other_term,
                BinOp::Sub => x -= other_term,
                _ => return Err(ParserError::Expected(Token::Op(BinOp::Add))),
            }
        }

        Ok(Expr::Num(x))
    }

    /// Attempts to parse a term
    pub fn term(&mut self) -> Result<Expr, ParserError> {
        // a will be a factor
        let mut a: i32 = self.factor()?.try_into()?;

        // Get the operator given the allowed tokens

        while !self.is_at_end() {
            let op = match self.matches(&[Token::Op(BinOp::Mul), Token::Op(BinOp::Div)]) {
                Some(op) => op,
                None => break,
            }
            .try_into_op()?;

            let b: i32 = self.factor()?.try_into()?;

            match op {
                BinOp::Mul => a *= b,
                BinOp::Div => a /= b,
                _ => panic!(),
            }
        }

        Ok(Expr::Num(a))
    }

    /// Attempts to parse a factor
    pub fn factor(&mut self) -> Result<Expr, ParserError> {
        if self.matches(&[Token::LeftBracket]).is_some() {
            let result = self.num_expr()?;

            if self.matches(&[Token::RightBracket]).is_none() {
                return Err(ParserError::Expected(Token::RightBracket));
            }

            return Ok(result);
        }

        Ok(Expr::Num(self.num()?))
    }

    /// Attempts to parse a number
    pub fn num(&mut self) -> Result<i32, ParserError> {
        if self.match_rule(&[Token::Int(0)]) {
            return Ok(self.prev().try_into_int()?);
        }

        Err(ParserError::Expected(Token::Int(0)))
    }
}
