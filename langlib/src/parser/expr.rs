use crate::{
    expr::{BinExpr, Expr},
    lexer::{
        op::{BinOp, UnOp},
        token::{Keyword, Token},
    },
};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse an expression
    pub fn expr(&mut self) -> Result<Expr, ParserError> {
        self.compare()
    }

    /// Attempts to parse a string token, and advances if successful.
    pub fn un_expr(&mut self) -> Result<Expr, ParserError> {
        // Check if there's a unary operator and then a left bracket
        if let Some(token) = self.matches(&[Token::UnOp(UnOp::Bang), Token::UnOp(UnOp::Minus)]) {
            if self.match_rule(&[Token::LeftBracket]) {
                let expr = self.expr()?;

                if !self.match_rule(&[Token::RightBracket]) {
                    return Err(ParserError::Expected(Token::RightBracket));
                }

                return Ok(Expr::Unary(token.try_into_un_op()?, Box::new(expr)));
            }
        }
        if self.match_rule(&[Token::LeftBracket]) {
            let expr = self.expr()?;

            if !self.match_rule(&[Token::RightBracket]) {
                return Err(ParserError::Expected(Token::RightBracket));
            }

            return Ok(expr);
        }

        unreachable!()
    }

    /// Attempts to parse a string token, and advances if successful.
    pub fn str_expr(&mut self) -> Result<Expr, ParserError> {
        if let Token::String(string) = self.curr() {
            self.adv();
            return Ok(Expr::Str(string));
        }

        Err(ParserError::ExpectedExpr)
    }

    /// Attempts to parse a variable token, and advances if successful.
    pub fn var_expr(&mut self) -> Result<Expr, ParserError> {
        if let Token::Ident(string) = self.curr() {
            self.adv();
            return Ok(Expr::Var(string));
        }

        Err(ParserError::ExpectedExpr)
    }

    /// Attempts to match a boolean token, and advances if successful.
    pub fn bool_expr(&mut self) -> Result<Expr, ParserError> {
        if let Some(bool_val) = self.matches(&[
            Token::Keyword(Keyword::True),
            Token::Keyword(Keyword::False),
        ]) {
            return Ok(bool_val.into_expr()?);
        }

        Err(ParserError::Expected(Token::Keyword(Keyword::True)))
    }
    /// Attempts to parse a comparision statement
    pub fn compare(&mut self) -> Result<Expr, ParserError> {
        let lhs = self
            .num_expr()
            .or_else(|_| self.var_expr())
            .or_else(|_| self.str_expr())
            .or_else(|_| self.bool_expr())
            .or_else(|_| self.un_expr())?;

        // Check if there's an equality sign or if we're at the end of the file, if so then return early.
        if self.is_at_end() || !self.match_rule(&[Token::EqSign]) {
            return Ok(lhs);
        }

        let rhs = self
            .num_expr()
            .or_else(|_| self.var_expr())
            .or_else(|_| self.str_expr())
            .or_else(|_| self.bool_expr())
            .or_else(|_| self.un_expr())?;

        Ok(Expr::Bin(BinExpr::new(
            Box::new(lhs),
            Box::new(rhs),
            BinOp::EqSign,
        )))
    }
}
