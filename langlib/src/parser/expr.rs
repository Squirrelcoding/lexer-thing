use crate::{
    expr::{BinExpr, Expr},
    lexer::{op::Op, token::Token},
};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse an expression
    pub fn expr(&mut self) -> Result<Expr, ParserError> {
        if self.match_rule(&[Token::LeftBracket]) {
            let expr = self.expr()?;

            if !self.match_rule(&[Token::RightBracket]) {
                return Err(ParserError::Expected(Token::RightBracket));
            }

            return Ok(expr);
        }


        self.compare()
    }

    /// Attempts to parse a string token, and advances if successful.
    pub fn str_expr(&mut self) -> Result<Expr, ParserError> {
        if let Token::String(string) = self.curr() {
            self.adv();
            return Ok(Expr::Str(string));
        }

        Err(ParserError::ExpectedExpr)
    }

    /// Attempts to match a boolean token, and advances if successful.
    pub fn bool_expr(&mut self) -> Result<Expr, ParserError> {
        if let Some(bool_val) = self.matches(&[
            Token::Keyword("true".to_owned()),
            Token::Keyword("false".to_owned()),
        ]) {
            return Ok(bool_val.into_expr()?);
        }

        Err(ParserError::Expected(Token::Keyword("".to_owned())))
    }
    /// Attempts to parse a comparision statement
    pub fn compare(&mut self) -> Result<Expr, ParserError> {
        let lhs = self
            .num_expr()
            .or_else(|_| self.str_expr())
            .or_else(|_| self.bool_expr())?;

        // Check if there's an equality sign, if not then return early.
        if !self.match_rule(&[Token::EqSign]) {

            return Ok(lhs);
        }

        let rhs = self
            .num_expr()
            .or_else(|_| self.str_expr())
            .or_else(|_| self.bool_expr())?;

        Ok(Expr::Bin(BinExpr::new(
            Box::new(lhs),
            Box::new(rhs),
            Op::EqSign,
        )))
    }
}
