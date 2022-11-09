use crate::{
    expr::{BinExpr, Expr},
    lexer::{
        op::{BinOp, UnOp},
        token::Token,
    },
};

use super::{error::ParserError, Parser};

impl Parser {
    pub fn expr_e(&mut self) -> Result<Expr, ParserError> {
        self.compare_e()
    }

    pub fn compare_e(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.comparision_e()?;

        while let Some(op) = self.matches(&[
            Token::Op(BinOp::EqSign),
            Token::Op(BinOp::NeqSign),
        ]) {
            let rhs = self.comparision_e()?;
            
            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    pub fn comparision_e(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.term_e()?;

        while let Some(op) = self.matches(&[
            Token::Op(BinOp::GreaterSign),
            Token::Op(BinOp::GreaterEqSign),
            Token::Op(BinOp::LessSign),
            Token::Op(BinOp::LessEqSign),
        ]) {
            let rhs = self.term_e()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    pub fn term_e(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.factor_e()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::Add), Token::Op(BinOp::Sub)]) {
            let rhs = self.factor_e()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    pub fn factor_e(&mut self) -> Result<Expr, ParserError> {
        let mut lhs = self.unary_e()?;

        while let Some(op) = self.matches(&[Token::Op(BinOp::Mul), Token::Op(BinOp::Div)]) {
            let rhs = self.unary_e()?;

            lhs = Expr::Bin(BinExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op.try_into_op()?,
            });
        }

        Ok(lhs)
    }

    pub fn unary_e(&mut self) -> Result<Expr, ParserError> {
        if self.match_rule(&[Token::UnOp(UnOp::Bang)]) {
            let expr = self.primary_e()?;
            return Ok(Expr::Unary(UnOp::Bang, Box::new(expr)));
        }

        self.primary_e()
    }

    pub fn primary_e(&mut self) -> Result<Expr, ParserError> {
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
                    println!("FEWHUISN");

                    Ok(Expr::Bool(false))
                }
                _ => Err(ParserError::UnexpectedToken(Token::Keyword(keyword))),
            },

            // Attempt to parse an expression wrapped in brackets
            _ => {
                if self.match_rule(&[Token::LeftBracket]) {
                    let expr = self.expr_e()?;
                    if !self.match_rule(&[Token::RightBracket]) {
                        return Err(ParserError::Expected(Token::RightBracket));
                    }

                    return Ok(expr);
                }

                return Err(ParserError::Expected(Token::LeftBracket));
            }
        }
    }
}

#[cfg(test)]
mod e_tests {
    use crate::{
        expr::{BinExpr, Expr},
        lexer::{
            op::{BinOp, UnOp},
            Lexer,
        },
        parser::Parser,
    };

    #[test]
    fn test_primary() {
        let bool_t = "true";
        let bool_f = "false";
        let int = "123";
        let str = "\"Hello!\"";

        // bool_t

        let bool_t_token = Lexer::new(bool_t).tokenize().unwrap();
        let bool_t_expr = Parser::new(bool_t_token).primary_e();

        assert!(bool_t_expr.is_ok());
        assert_eq!(bool_t_expr.unwrap(), Expr::Bool(true));

        // bool_f

        let bool_f_token = Lexer::new(bool_f).tokenize().unwrap();
        let bool_f_expr = Parser::new(bool_f_token).primary_e();

        assert!(bool_f_expr.is_ok());
        assert_eq!(bool_f_expr.unwrap(), Expr::Bool(false));

        // int

        let int_token = Lexer::new(int).tokenize().unwrap();
        let int_token_expr = Parser::new(int_token).primary_e();

        assert!(int_token_expr.is_ok());
        assert_eq!(int_token_expr.unwrap(), Expr::Num(123));

        // str

        let str_token = Lexer::new(str).tokenize().unwrap();
        let str_expr = Parser::new(str_token).primary_e();

        assert!(str_expr.is_ok());
        assert_eq!(str_expr.unwrap(), Expr::Str("Hello!".to_owned()));
    }

    #[test]
    fn test_unary() {
        // Test for the expression "!true".

        let expr = "!true";

        let tokens = Lexer::new(expr).tokenize().unwrap();

        let expr = Parser::new(tokens).unary_e();

        assert!(expr.is_ok());
        assert_eq!(
            expr.unwrap(),
            Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true)))
        );

        // Test for the expression !"Hello!", despite the fact that this makes no sense in practice.

        let expr = "\"Hello!\"";

        let tokens = Lexer::new(expr).tokenize().unwrap();
        let expr = Parser::new(tokens).unary_e();

        assert!(expr.is_ok());
        assert_eq!(expr.unwrap(), Expr::Str("Hello!".to_owned()));

        // Test for the expression !(((((((1234))))))), despite the fact that this, again, makes no sense.

        let expr = "!(((((((1234)))))))";

        let tokens = Lexer::new(expr).tokenize().unwrap();
        let expr = Parser::new(tokens).unary_e();

        assert!(expr.is_ok());
        assert_eq!(
            expr.unwrap(),
            Expr::Unary(UnOp::Bang, Box::new(Expr::Num(1234)))
        );
    }

    #[test]
    fn test_factor() {
        let s = "!true * false";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let expr = Parser::new(tokens).factor_e();

        assert!(expr.is_ok());
        assert_eq!(
            expr.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true)))),
                rhs: Box::new(Expr::Bool(false)),
                op: BinOp::Mul
            })
        );

        // An expression which makes absolutely no sense.
        let s = "!true / (\"Some string\")";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let expr = Parser::new(tokens).factor_e();

        assert!(expr.is_ok());
        assert_eq!(
            expr.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true)))),
                rhs: Box::new(Expr::Str("Some string".to_owned())),
                op: BinOp::Div
            })
        );

        let s = "1 + 1";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let expr = Parser::new(tokens).factor_e();

        assert!(expr.is_ok());

        let s = "25";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let expr = Parser::new(tokens).factor_e();

        assert!(expr.is_ok());
        assert_eq!(expr.unwrap(), Expr::Num(25))
    }

    #[test]
    fn test_comparision_multiple() {
        let s = "12 > 43 <= 324";

        let tokens = Lexer::new(s).tokenize().unwrap();
        let result = Parser::new(tokens).comparision_e();

        assert_eq!(
            result.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Num(12)),
                    rhs: Box::new(Expr::Num(43)),
                    op: BinOp::GreaterSign
                })),
                rhs: Box::new(Expr::Num(324)),
                op: BinOp::LessEqSign
            })
        );
    }

    #[test]
    fn test_comparision() {
        let s_1 = "32 + 324 / 23 + 25 / 234 - 234 >= 234 - 243 + 4232";

        let tokens = Lexer::new(s_1).tokenize().unwrap();
        let expr = Parser::new(tokens).comparision_e();

        assert_eq!(
            expr.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(32)),
                            rhs: Box::new(Expr::Bin(BinExpr {
                                lhs: Box::new(Expr::Num(324)),
                                rhs: Box::new(Expr::Num(23)),
                                op: BinOp::Div
                            })),
                            op: BinOp::Add
                        })),
                        rhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(25)),
                            rhs: Box::new(Expr::Num(234)),
                            op: BinOp::Div
                        })),
                        op: BinOp::Add
                    })),
                    rhs: Box::new(Expr::Num(234)),
                    op: BinOp::Sub
                })),
                rhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Num(234)),
                        rhs: Box::new(Expr::Num(243)),
                        op: BinOp::Sub
                    })),
                    rhs: Box::new(Expr::Num(4232)),
                    op: BinOp::Add
                })),
                op: BinOp::GreaterEqSign
            })
        );

        let s_2 = "32 + 324 / 23 + 25 / 234 - 234 > 234 - 243 + 4232";

        let tokens = Lexer::new(s_2).tokenize().unwrap();
        let expr = Parser::new(tokens).comparision_e();

        assert_eq!(
            expr.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(32)),
                            rhs: Box::new(Expr::Bin(BinExpr {
                                lhs: Box::new(Expr::Num(324)),
                                rhs: Box::new(Expr::Num(23)),
                                op: BinOp::Div
                            })),
                            op: BinOp::Add
                        })),
                        rhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(25)),
                            rhs: Box::new(Expr::Num(234)),
                            op: BinOp::Div
                        })),
                        op: BinOp::Add
                    })),
                    rhs: Box::new(Expr::Num(234)),
                    op: BinOp::Sub
                })),
                rhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Num(234)),
                        rhs: Box::new(Expr::Num(243)),
                        op: BinOp::Sub
                    })),
                    rhs: Box::new(Expr::Num(4232)),
                    op: BinOp::Add
                })),
                op: BinOp::GreaterSign
            })
        );

        let s_3 = "32 + 324 / 23 + 25 / 234 - 234 <= 234 - 243 + 4232";

        let tokens = Lexer::new(s_3).tokenize().unwrap();
        let expr = Parser::new(tokens).comparision_e();

        assert_eq!(
            expr.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(32)),
                            rhs: Box::new(Expr::Bin(BinExpr {
                                lhs: Box::new(Expr::Num(324)),
                                rhs: Box::new(Expr::Num(23)),
                                op: BinOp::Div
                            })),
                            op: BinOp::Add
                        })),
                        rhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(25)),
                            rhs: Box::new(Expr::Num(234)),
                            op: BinOp::Div
                        })),
                        op: BinOp::Add
                    })),
                    rhs: Box::new(Expr::Num(234)),
                    op: BinOp::Sub
                })),
                rhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Num(234)),
                        rhs: Box::new(Expr::Num(243)),
                        op: BinOp::Sub
                    })),
                    rhs: Box::new(Expr::Num(4232)),
                    op: BinOp::Add
                })),
                op: BinOp::LessEqSign
            })
        );

        let s_4 = "32 + 324 / 23 + 25 / 234 - 234 < 234 - 243 + 4232";

        let tokens = Lexer::new(s_4).tokenize().unwrap();
        let expr = Parser::new(tokens).comparision_e();

        assert_eq!(
            expr.unwrap(),
            Expr::Bin(BinExpr {
                lhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(32)),
                            rhs: Box::new(Expr::Bin(BinExpr {
                                lhs: Box::new(Expr::Num(324)),
                                rhs: Box::new(Expr::Num(23)),
                                op: BinOp::Div
                            })),
                            op: BinOp::Add
                        })),
                        rhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(25)),
                            rhs: Box::new(Expr::Num(234)),
                            op: BinOp::Div
                        })),
                        op: BinOp::Add
                    })),
                    rhs: Box::new(Expr::Num(234)),
                    op: BinOp::Sub
                })),
                rhs: Box::new(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Num(234)),
                        rhs: Box::new(Expr::Num(243)),
                        op: BinOp::Sub
                    })),
                    rhs: Box::new(Expr::Num(4232)),
                    op: BinOp::Add
                })),
                op: BinOp::LessSign
            })
        );
    }
}
