use super::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    Declaration(Declaration),
    Assignment(Declaration),
    Print(Expr),
    Expr(Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum StmtErr {
    #[error("A failed conversion occured")]
    FailedConversion,

    #[error("An unknown keyword has been encountered. I don't even know how this is supposed to happen.")]
    UnknownKeyword,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Declaration {
    pub ident: String,
    pub val: Expr,
}

#[cfg(test)]
mod stmt_tests {
    use crate::{
        expr::{BinExpr, Expr},
        lexer::{
            op::BinOp,
            token::{Keyword, Token},
        },
        parser::Parser,
        stmt::Declaration,
    };

    use super::Stmt;

    #[test]
    fn successful_let_stmt() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Ident("coolVariable".to_owned()),
            Token::DeclarationSign,
            Token::LeftBracket,
            Token::Int(1),
            Token::Op(BinOp::Add),
            Token::Int(1),
            Token::RightBracket,
            Token::Semi,
        ];

        let binding = Parser::new(tokens).stmt();

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_eq!(
            binding,
            Stmt::Declaration(Declaration {
                ident: "coolVariable".to_owned(),
                val: Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Num(1)),
                    rhs: Box::new(Expr::Num(1)),
                    op: BinOp::Add
                })
            })
        )
    }

    #[test]
    fn successful_bool_declaration() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Ident("coolVariable".to_owned()),
            Token::DeclarationSign,
            Token::LeftBracket,
            Token::Keyword(Keyword::True),
            Token::Op(BinOp::EqSign),
            Token::Keyword(Keyword::False),
            Token::RightBracket,
            Token::Semi,
        ];

        let binding = Parser::new(tokens).stmt();

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_eq!(
            binding,
            Stmt::Declaration(Declaration {
                ident: "coolVariable".to_owned(),
                val: Expr::Bin(BinExpr {
                    lhs: Box::new(Expr::Bool(true)),
                    rhs: Box::new(Expr::Bool(false)),
                    op: BinOp::EqSign
                })
            })
        )
    }

    #[test]
    fn bad_let_stmt() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Ident("coolVariable".to_owned()),
            Token::DeclarationSign,
            Token::LeftBracket,
            Token::Int(1),
            Token::Op(BinOp::Add),
            Token::Int(1),
            Token::RightBracket,
            Token::Semi,
        ];

        let binding = Parser::new(tokens).stmt();

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_ne!(
            binding,
            Stmt::Declaration(Declaration {
                ident: "coolVariable".to_owned(),
                val: Expr::Num(3)
            })
        )
    }
}
