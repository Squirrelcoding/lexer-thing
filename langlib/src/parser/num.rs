use crate::{
    expr::{Expr, BinExpr},
    lexer::{op::BinOp, token::Token},
};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse an arithmetic expression
    pub fn num_expr(&mut self) -> Result<Expr, ParserError> {
        let lhs = self.term()?;

        while !self.is_at_end() {
            let op = match self.matches(&[Token::Op(BinOp::Add), Token::Op(BinOp::Sub)]) {
                Some(op) => op,
                None => break,
            }
            .try_into_op()?;

            let rhs = self.term()?;

            return match op {
                BinOp::Add | BinOp::Sub => {
                    Ok(Expr::Bin(BinExpr {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op,
                    }))
                }
                _ => panic!(),
            };
        }

        Ok(lhs)
    }

    /// Attempts to parse a term
    pub fn term(&mut self) -> Result<Expr, ParserError> {
        // lhs will be a factor
        let lhs = self.factor()?;

        // Get the operator, which can either be a * or a /.
        while !self.is_at_end() {
            let op = match self.matches(&[Token::Op(BinOp::Mul), Token::Op(BinOp::Div)]) {
                Some(op) => op,
                None => break,
            }
            .try_into_op()?;

            let rhs = self.factor()?;

            return match op {
                BinOp::Mul | BinOp::Div => {
                    Ok(Expr::Bin(BinExpr {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op,
                    }))
                }
                _ => panic!(),
            };
        }

        Ok(lhs)
    }

    /// Attempts to parse a factor, returns a `Grouping` or a literal that can either be a number or an identifier.
    pub fn factor(&mut self) -> Result<Expr, ParserError> {
        if self.matches(&[Token::LeftBracket]).is_some() {
            let result = self.num_expr()?;

            if self.matches(&[Token::RightBracket]).is_none() {
                return Err(ParserError::Expected(Token::RightBracket));
            }

            return Ok(Expr::Grouping(Box::new(result)));
        }

        Ok(self.num_var()?)
    }

    /// Attempts to parse a number / identifier.
    pub fn num_var(&mut self) -> Result<Expr, ParserError> {

        let result = match self.curr() {
            Token::Int(int) => Expr::Num(int),
            Token::Ident(ident) => Expr::Var(ident),
            _ => return Err(ParserError::Expected(Token::Int(0)))
        };

        // Increment the cursor
        self.adv();

        Ok(result)
    }
}
