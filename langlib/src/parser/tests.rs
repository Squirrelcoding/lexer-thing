#[cfg(test)]
/// A module for parser unit tests.
mod parser_tests {

    use crate::{
        expr::{BinExpr, Expr},
        lexer::{
            op::{BinOp, UnOp},
            token::{Keyword, Token},
            Lexer,
        },
        parser::Parser,
        stmt::{Declaration, Stmt},
    };

    #[test]
    fn test_helpers() {
        let s = "let a = (1 + 1) + 2 - 432; let b = 3;";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let fourth = parser.at(4);

        assert!(fourth.is_ok());

        let fourth = fourth.unwrap();

        assert_eq!(fourth, Token::Int(1));
        assert_eq!(parser.curr(), Ok(Token::Keyword(Keyword::Let)));

        parser.adv();

        assert_eq!(parser.prev(), Ok(Token::Keyword(Keyword::Let)));

        (1..(parser.tokens.len() - 1)).for_each(|_| {
            parser.adv();
        });

        assert_eq!(parser.pos(), parser.tokens.len() + 1);
    }

    #[test]
    fn match_rule_success() {
        let a = "let x = 5;";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let rules = [
            Token::Keyword(Keyword::Let),
            Token::Ident("x".to_owned()),
            Token::DeclarationSign,
            Token::Int(5),
        ];

        assert!(parser.match_rule(&rules));
    }

    #[test]
    fn match_rule_fail() {
        let a = "let = 5;";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let rules = [
            Token::Keyword(Keyword::Let),
            Token::Ident("x".to_owned()),
            Token::DeclarationSign,
            Token::Int(0),
            Token::Semi,
        ];

        assert!(!parser.match_rule(&rules));
    }

    #[test]
    fn matches_success() {
        let pm = [Token::Op(BinOp::Add), Token::Op(BinOp::Sub)];

        let a = "+ - 3";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        assert_eq!(parser.matches(&pm), Some(Token::Op(BinOp::Add)));
        assert_eq!(parser.matches(&pm), Some(Token::Op(BinOp::Sub)));
        assert_eq!(parser.matches(&pm), None)
    }

    #[test]
    fn test_compare_nums_success() {
        let s = "(3 + 15) / 2 == 9";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.equality();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.eval().is_ok());
    }

    #[test]
    fn test_compare_nums_fail() {
        let s = "(3 + 15) / 2 == 20";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.equality();

        assert!(result.is_ok());

        let result = result.unwrap().eval();
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    fn test_compare_strs_success() {
        let s = " \"This is a string\" == \"This is a string\"";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.equality();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result, Expr::Bool(true));
    }

    #[test]
    fn test_compare_strs_fail() {
        let s = " \"This is a string\" == \"This is another string\"";

        println!("================================================");

        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.equality();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    pub fn compare_bools_success() {
        let s = "true == true";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.equality();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result, Expr::Bool(true));
    }

    #[test]
    pub fn compare_bools_fail() {
        let s = "true == false";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.equality();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    pub fn test_unary_negation() {
        let s = "let x = !(true == false);";

        let mut lexer = Lexer::new(s);

        let binding_stmt = Parser::new(lexer.tokenize().unwrap()).stmt();

        assert!(binding_stmt.is_ok());

        let binding_stmt = binding_stmt.unwrap();

        assert_eq!(
            binding_stmt,
            Stmt::Declaration(Declaration {
                ident: "x".to_owned(),
                val: Expr::Unary(
                    UnOp::Bang,
                    Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bool(true)),
                        rhs: Box::new(Expr::Bool(false)),
                        op: BinOp::EqSign
                    }))
                )
            })
        );
    }

    #[test]
    pub fn test_unary_negation_but_with_strings() {
        let s = "let x = !(\"this is a string.\" == \"this is another string.\");";

        let mut lexer = Lexer::new(s);

        let binding_stmt = Parser::new(lexer.tokenize().unwrap()).stmt();

        assert!(binding_stmt.is_ok());

        let binding_stmt = binding_stmt.unwrap();

        assert_eq!(
            binding_stmt,
            Stmt::Declaration(Declaration {
                ident: "x".to_owned(),
                val: Expr::Unary(
                    UnOp::Bang,
                    Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Str("this is a string.".to_owned())),
                        rhs: Box::new(Expr::Str("this is another string.".to_owned())),
                        op: BinOp::EqSign
                    }))
                )
            })
        );
    }

    #[test]
    pub fn test_parse_statements() {
        let s = "let    
    
        x =   !        (\"this is a string.\"                == \"this is another string.\"); print (23-5)/ 2; let y =  (           2 + 4) / 2; let z = !   true; 
        
        
        
        print 
        
        
        \"This is a very cool string.\"; let undefinedVar;";

        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let statements = parser.get_statements();

        assert!(statements.is_ok());

        let statements = statements.unwrap();

        assert_eq!(
            statements,
            vec![
                Stmt::Declaration(Declaration {
                    ident: "x".to_owned(),
                    val: Expr::Unary(
                        UnOp::Bang,
                        Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Str("this is a string.".to_owned())),
                            rhs: Box::new(Expr::Str("this is another string.".to_owned())),
                            op: BinOp::EqSign
                        }))
                    )
                }),
                Stmt::Print(Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Num(23)),
                        rhs: Box::new(Expr::Num(5)),
                        op: BinOp::Sub
                    })),
                    rhs: Box::new(Expr::Num(2)),
                    op: BinOp::Div
                })),
                Stmt::Declaration(Declaration {
                    ident: "y".to_owned(),
                    val: Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(2)),
                            rhs: Box::new(Expr::Num(4)),
                            op: BinOp::Add
                        })),
                        rhs: Box::new(Expr::Num(2)),
                        op: BinOp::Div
                    })
                }),
                Stmt::Declaration(Declaration {
                    ident: "z".to_owned(),
                    val: Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true)))
                }),
                Stmt::Print(Expr::Str("This is a very cool string.".to_owned())),
                Stmt::Declaration(Declaration {
                    ident: "undefinedVar".to_owned(),
                    val: Expr::Null
                })
            ]
        )
    }

    #[test]
    fn test_if_stmt() {
        let s = "
        if (2 == 5) {
            print \"2 is equal to 5\";
            print \"The univese makes no sense.\";
        }
        ";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(tokens).stmt();

        assert_eq!(
            ast.unwrap(),
            Stmt::If(
                Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Num(2)),
                    rhs: Box::new(Expr::Num(5)),
                    op: BinOp::EqSign
                }),
                Box::new(Stmt::Block(vec![
                    Stmt::Print(Expr::Str("2 is equal to 5".to_owned())),
                    Stmt::Print(Expr::Str("The univese makes no sense.".to_owned()))
                ])),
                None
            )
        );
    }

    #[test]
    fn test_if_stmt_with_else_block() {
        let s = "
        if (2 == 5) {
            print \"2 is equal to 5\";
            print \"The univese makes no sense.\";
        } else {
            print \"2 is NOT equal to 5.\";
            print \"As it should be.\";
        }
        ";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(tokens).stmt();

        assert_eq!(
            ast.unwrap(),
            Stmt::If(
                Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Num(2)),
                    rhs: Box::new(Expr::Num(5)),
                    op: BinOp::EqSign
                }),
                Box::new(Stmt::Block(vec![
                    Stmt::Print(Expr::Str("2 is equal to 5".to_owned())),
                    Stmt::Print(Expr::Str("The univese makes no sense.".to_owned()))
                ])),
                Some(Box::new(Stmt::Block(vec![
                    Stmt::Print(Expr::Str("2 is NOT equal to 5.".to_owned())),
                    Stmt::Print(Expr::Str("As it should be.".to_owned()))
                ])))
            )
        );
    }

    #[test]
    fn test_lonely_if_statement() {
        let s = "
        if (2 == 5) print \"We just broke the laws of the universe.\";
        ";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(tokens).stmt();

        assert_eq!(
            ast.unwrap(),
            Stmt::If(
                Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Num(2)),
                    rhs: Box::new(Expr::Num(5)),
                    op: BinOp::EqSign
                }),
                Box::new(Stmt::Print(Expr::Str(
                    "We just broke the laws of the universe.".to_owned()
                ))),
                None
            )
        );
    }

    #[test]
    fn test_single_nested_stmt() {
        let s = "
        {
            print \"You can nest statements!\";
        }
    ";

        let tokens = Lexer::new(s).tokenize().unwrap();

        let ast = Parser::new(tokens).get_statements();

        assert_eq!(
            ast.unwrap(),
            vec![Stmt::Block(vec![Stmt::Print(Expr::Str(
                "You can nest statements!".to_owned()
            ))])]
        );
    }

    #[test]
    fn test_multiple_nested_if_stmt() {
        let s = "
        {
            {
                {
                    if (true) {
                        print \"You can nest statements!\";
                    } else {
                        print \"This won't be reached but it's here anyway!\";
                    }
                }
            }
        }
    ";

        let tokens = Lexer::new(s).tokenize().unwrap();

        let ast = Parser::new(tokens).get_statements();

        assert_eq!(
            ast.unwrap(),
            [Stmt::Block(vec![Stmt::Block(vec![Stmt::Block(vec![
                Stmt::If(
                    Expr::Bool(true),
                    Box::new(Stmt::Block(vec![Stmt::Print(Expr::Str(
                        "You can nest statements!".to_owned()
                    ))])),
                    Some(Box::new(Stmt::Block(vec![Stmt::Print(Expr::Str(
                        "This won't be reached but it's here anyway!".to_owned()
                    ))])))
                )
            ])])])]
        );
    }

    #[test]
    fn test_while_loop() {
        let s = "
        while (5 == 5) {
            if (true or false) {
                print \"HERE\";
            }
        }
    ";

        let tokens = Lexer::new(s).tokenize().unwrap();

        let ast = Parser::new(tokens).get_statements();

        assert_eq!(
            ast.unwrap(),
            [Stmt::While(
                Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Num(5)),
                    rhs: Box::new(Expr::Num(5)),
                    op: BinOp::EqSign
                }),
                Box::new(Stmt::Block(vec![Stmt::If(
                    Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bool(true)),
                        rhs: Box::new(Expr::Bool(false)),
                        op: BinOp::Or
                    }),
                    Box::new(Stmt::Block(vec![Stmt::Print(Expr::Str("HERE".to_owned()))])),
                    None
                )]))
            )]
        );
    }

    #[test]
    fn test_simple_function_call() {
        let s = "someFunction();";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(tokens).expr();

        assert_eq!(
            ast.unwrap(),
            Expr::Funcall(Box::new(Expr::Var("someFunction".to_owned())), vec![])
        );
    }

    #[test]
    fn test_function_call_with_args() {
        let s = "someFunction((123456789 * 2) / 3, \"A very cool string\", !true, a * 2);";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(tokens).expr();

        assert_eq!(
            ast.unwrap(),
            Expr::Funcall(
                Box::new(Expr::Var("someFunction".to_owned())),
                vec![
                    Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Bin(BinExpr {
                            lhs: Box::new(Expr::Num(123456789)),
                            rhs: Box::new(Expr::Num(2)),
                            op: BinOp::Mul
                        })),
                        rhs: Box::new(Expr::Num(3)),
                        op: BinOp::Div
                    }),
                    Expr::Str("A very cool string".to_owned()),
                    Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true))),
                    Expr::Bin(BinExpr {
                        lhs: Box::new(Expr::Var("a".to_owned())),
                        rhs: Box::new(Expr::Num(2)),
                        op: BinOp::Mul
                    })
                ]
            )
        );
    }

    #[test]
    fn test_crazy_function_call() {
        let s = "someFunction((123456789 * 2) / 3, \"A very cool string\", !true, a * 2)(32, 65, 21)(a, b, c);";

        let tokens = Lexer::new(s).tokenize().unwrap();

        let expr = Parser::new(tokens).expr();

        assert_eq!(
            expr.unwrap(),
            Expr::Funcall(
                Box::new(Expr::Funcall(
                    Box::new(Expr::Funcall(
                        Box::new(Expr::Var("someFunction".to_string())),
                        vec![
                            Expr::Bin(BinExpr {
                                lhs: Box::new(Expr::Bin(BinExpr {
                                    lhs: Box::new(Expr::Num(123456789)),
                                    rhs: Box::new(Expr::Num(2)),
                                    op: BinOp::Mul
                                })),
                                rhs: Box::new(Expr::Num(3)),
                                op: BinOp::Div
                            }),
                            Expr::Str("A very cool string".to_owned()),
                            Expr::Unary(UnOp::Bang, Box::new(Expr::Bool(true))),
                            Expr::Bin(BinExpr {
                                lhs: Box::new(Expr::Var("a".to_string())),
                                rhs: Box::new(Expr::Num(2)),
                                op: BinOp::Mul
                            })
                        ]
                    )),
                    vec![Expr::Num(32), Expr::Num(65), Expr::Num(21)]
                )),
                vec![
                    Expr::Var("a".to_owned()),
                    Expr::Var("b".to_owned()),
                    Expr::Var("c".to_owned())
                ]
            )
        );
    }
}
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
