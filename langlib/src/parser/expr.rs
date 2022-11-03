use crate::{expr::Expr, lexer::token::Token};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse an expression
    pub fn expr(&mut self) -> Result<Expr, ParserError> {
        self.num_expr()
            .or_else(|_| self.str_expr())
            .or_else(|_| self.bool_expr())
    }

    /// Attempts to parse a string token, and advances if successful.
    pub fn str_expr(&mut self) -> Result<Expr, ParserError> {
        if let Token::String(string) = self.curr() {
            self.adv();
            return Ok(Expr::Str(string));
        }

        Err(ParserError::Expected(Token::String("".to_owned())))
    }

    /// Attempts to match a boolean token, and advances if successful.
    pub fn bool_expr(&mut self) -> Result<Expr, ParserError> {
        if let Some(bool_val) = self.matches(&[
            Token::Keyword("true".to_owned()),
            Token::Keyword("false".to_owned()),
        ]) {
            return Ok(bool_val.into_expr()?);
        }

        Err(ParserError::Expected(Token::String("".to_owned())))
    }
    /// Attempts to parse a comparision statement
    pub fn compare(&mut self) -> Result<Expr, ParserError> {
        let lhs = self.expr()?;

        // Check if there's an equality sign
        if !self.match_rule(&[Token::EqSign]) {
            return Err(ParserError::Expected(Token::EqSign));
        }

        let rhs = self.expr()?;

        Ok(Expr::Comparison(Box::new(lhs), Box::new(rhs)))
    }
}
