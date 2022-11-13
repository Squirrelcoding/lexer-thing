#[cfg(test)]
mod expr_tests {
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
        let bool_txpr = Parser::new(bool_t_token).primary();

        assert!(bool_txpr.is_ok());
        assert_eq!(bool_txpr.unwrap(), Expr::Bool(true));

        // bool_f

        let bool_f_token = Lexer::new(bool_f).tokenize().unwrap();
        let bool_fxpr = Parser::new(bool_f_token).primary();

        assert!(bool_fxpr.is_ok());
        assert_eq!(bool_fxpr.unwrap(), Expr::Bool(false));

        // int

        let int_token = Lexer::new(int).tokenize().unwrap();
        let int_tokenxpr = Parser::new(int_token).primary();

        assert!(int_tokenxpr.is_ok());
        assert_eq!(int_tokenxpr.unwrap(), Expr::Num(123));

        // str

        let str_token = Lexer::new(str).tokenize().unwrap();
        let strxpr = Parser::new(str_token).primary();

        assert!(strxpr.is_ok());
        assert_eq!(strxpr.unwrap(), Expr::Str("Hello!".to_owned()));
    }

    #[test]
    fn test_unary() {
        // Test for the expression "!true".

        let expr = "!true";

        let tokens = Lexer::new(expr).tokenize().unwrap();

        let expr = Parser::new(tokens).unary();

        assert!(expr.is_ok());
        assert_eq!(
            expr.unwrap(),
            Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true)))
        );

        // Test for the expression !"Hello!", despite the fact that this makes no sense in practice.

        let expr = "\"Hello!\"";

        let tokens = Lexer::new(expr).tokenize().unwrap();
        let expr = Parser::new(tokens).unary();

        assert!(expr.is_ok());
        assert_eq!(expr.unwrap(), Expr::Str("Hello!".to_owned()));

        // Test for the expression !(((((((1234))))))), despite the fact that this, again, makes no sense.

        let expr = "!(((((((1234)))))))";

        let tokens = Lexer::new(expr).tokenize().unwrap();
        let expr = Parser::new(tokens).unary();

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
        let expr = Parser::new(tokens).factor();

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
        let expr = Parser::new(tokens).factor();

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
        let expr = Parser::new(tokens).factor();

        assert!(expr.is_ok());

        let s = "25";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let expr = Parser::new(tokens).factor();

        assert!(expr.is_ok());
        assert_eq!(expr.unwrap(), Expr::Num(25))
    }

    #[test]
    fn test_comparision_multiple() {
        let s = "12 > 43 <= 324";

        let tokens = Lexer::new(s).tokenize().unwrap();
        let result = Parser::new(tokens).comparision();

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
        let expr = Parser::new(tokens).comparision();

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
        let expr = Parser::new(tokens).comparision();

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
        let expr = Parser::new(tokens).comparision();

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
        let expr = Parser::new(tokens).comparision();

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
