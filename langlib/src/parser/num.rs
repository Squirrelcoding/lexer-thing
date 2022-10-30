use crate::lexer::{op::Op, token::Token};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse an arithmetic expression
    pub fn num_expr(&mut self) -> Result<i32, ParserError> {
        let mut x = self.term()?;

        while !self.is_at_end() {
            let op = match self.matches(&[Token::Op(Op::Add), Token::Op(Op::Sub)]) {
                Some(op) => op,
                None => break,
            }
            .op()?;

            let other_term = self.term()?;
            match op {
                Op::Add => x += other_term,
                Op::Sub => x -= other_term,
                _ => return Err(ParserError::Expected(Token::Op(Op::Add))),
            }
        }

        Ok(x)
    }

    /// Attempts to parse a term
    pub fn term(&mut self) -> Result<i32, ParserError> {
        // a will be a factor
        let mut a = self.factor()?;

        // Get the operator given the allowed tokens

        while !self.is_at_end() {
            let op = match self.matches(&[Token::Op(Op::Mul), Token::Op(Op::Div)]) {
                Some(op) => op,
                None => break,
            }
            .op()?;

            let b = self.factor()?;

            match op {
                Op::Mul => a *= b,
                Op::Div => a /= b,
                _ => panic!(),
            }
        }

        Ok(a)
    }

    /// Attempts to parse a factor
    pub fn factor(&mut self) -> Result<i32, ParserError> {
        if self.matches(&[Token::LeftBracket]).is_some() {
            let result = self.num_expr()?;

            if self.matches(&[Token::RightBracket]).is_none() {
                return Err(ParserError::Expected(Token::RightBracket));
            }

            return Ok(result);
        }

        self.num()
    }

    /// Attempts to parse a number
    pub fn num(&mut self) -> Result<i32, ParserError> {
        if self.match_rule(&[Token::Int(0)]) {
            return Ok(self.prev().try_into_int()?);
        }

        Err(ParserError::BadTerm)
    }
}
