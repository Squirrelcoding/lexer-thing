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

        assert_eq!(parser.pos(), parser.tokens.len() - 1);
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
            Token::Semi,
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

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.eval().is_ok());
    }

    #[test]
    fn test_compare_nums_fail() {
        let s = "(3 + 15) / 2 == 20";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();

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

        let result = parser.compare();
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

        let result = parser.compare();
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

        let result = parser.compare();
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

        let result = parser.compare();
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
            Stmt::IfStmt(
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
            Stmt::IfStmt(
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
            Stmt::IfStmt(
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
}
