use crate::{
    expr::{BinExpr, Expr},
    lexer::{
        op::{BinOp, UnOp},
        token::Token,
    },
};

use super::{err::ParserError, Parser};

impl Parser {
    /// Attempts to parse an expression.
    pub fn expr(&mut self) -> Result<Expr, ParserError> {
        self.logical_or()
    }

    /// Attempts to parse a logical `or` expession.
    pub fn logical_or(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.logical_and()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::Or)]) {
            let rhs = self.logical_and()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    /// Attempts to parse a logical `and` expession.
    pub fn logical_and(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.equality()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::And)]) {
            let rhs = self.equality()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    /// Attempts to parse a compare expression.
    pub fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.comparision()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::EqSign), Token::Op(BinOp::NeqSign)]) {
            let rhs = self.comparision()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    /// Attempts to parse a comparision expression.
    pub fn comparision(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.term()?;

        while let Some(op) = self.matches(&[
            Token::Op(BinOp::GreaterSign),
            Token::Op(BinOp::GreaterEqSign),
            Token::Op(BinOp::LessSign),
            Token::Op(BinOp::LessEqSign),
        ]) {
            let rhs = self.term()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    /// Attempts to parse a term.
    pub fn term(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.factor()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::Add), Token::Op(BinOp::Sub)]) {
            let rhs = self.factor()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    /// Attempts to parse a factor.
    pub fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.unary()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::Mul), Token::Op(BinOp::Div)]) {
            let rhs = self.unary()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    /// Attempts to parse a unary expression.
    pub fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_rule(&[Token::UnOp(UnOp::Bang)]) {
            let expr = self.primary()?;
            return Ok(Expr::Unary(UnOp::Bang, Box::new(expr)));
        }

        self.primary()
    }

    /// Attempts to parse a "primary". A primary is a type similiar to a literal, however a primary can include things
    /// such as expressions wrapped in parent or an identifier.
    pub fn primary(&mut self) -> Result<Expr, ParserError> {
        match self.curr()? {
            Token::Int(int) => {
                self.adv();
                Ok(Expr::Num(int))
            }
            Token::String(str) => {
                self.adv();

                Ok(Expr::Str(str))
            }
            Token::Ident(ident) => {
                self.adv();

                Ok(Expr::Var(ident))
            }
            Token::Keyword(keyword) => match keyword {
                crate::lexer::token::Keyword::True => {
                    self.adv();

                    Ok(Expr::Bool(true))
                }
                crate::lexer::token::Keyword::False => {
                    self.adv();

                    Ok(Expr::Bool(false))
                }
                _ => Err(ParserError::UnexpectedToken(Token::Keyword(keyword))),
            },

            // Attempt to parse an expression wrapped in brackets
            _ => {
                if self.match_rule(&[Token::LeftBracket]) {
                    let expr = self.expr()?;
                    if !self.match_rule(&[Token::RightBracket]) {
                        return Err(ParserError::Expected(Token::RightBracket, self.cursor));
                    }

                    return Ok(expr);
                }

                Err(ParserError::Expected(Token::LeftBracket, self.cursor))
            }
        }
    }
}
