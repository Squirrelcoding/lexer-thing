use crate::{
    lexer::token::{Keyword, Token},
    parser::{error::ParserError, Parser},
};

use super::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    Declaration(Declaration),
    Print(Expr),
    ExprStatement(Expr),
}

impl Stmt {
    #[allow(clippy::single_match)]
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, ParserError> {
        match &tokens[0] {
            Token::Keyword(keyword) => match keyword {
                Keyword::Let => {
                    // let statements must be at least 4 tokens long.
                    if tokens.len() < 4 {
                        return Err(ParserError::InvalidLetStatement);
                    }

                    // Check for identifier
                    let ident = match tokens[1].clone() {
                        Token::Ident(ident) => ident,
                        _ => return Err(ParserError::Expected(Token::Ident("".to_owned()))),
                    };

                    // Check if there is an Declaration sign.
                    if Token::DeclarationSign != tokens[2] {
                        return Err(ParserError::Expected(Token::DeclarationSign));
                    }

                    let expr = Parser::new(tokens[3..].to_vec()).expr()?;

                    Ok(Self::Declaration(Declaration { ident, val: expr }))
                }
                _ => Err(ParserError::StmtErr(StmtErr::UnknownKeyword)),
            },
            _ => {
                let expr = Parser::new(tokens.to_vec()).expr()?;

                Ok(Stmt::ExprStatement(expr))
            }
        }
    }
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
        expr::{Expr, BinExpr},
        lexer::{
            op::BinOp,
            token::{Keyword, Token},
        },
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

        let binding = Stmt::from_tokens(&tokens);

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_eq!(
            binding,
            Stmt::Declaration(Declaration {
                ident: "coolVariable".to_owned(),
                val: Expr::Bin(BinExpr { lhs: Box::new(Expr::Num(1)), rhs: Box::new(Expr::Num(1)), op: BinOp::Add })
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
            Token::EqSign,
            Token::Keyword(Keyword::False),
            Token::RightBracket,
            Token::Semi,
        ];

        let binding = Stmt::from_tokens(&tokens);

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_eq!(
            binding,
            Stmt::Declaration(Declaration {
                ident: "coolVariable".to_owned(),
                val: Expr::Bin(BinExpr { lhs: Box::new(Expr::Bool(true)), rhs: Box::new(Expr::Bool(false)), op: BinOp::EqSign })
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

        let binding = Stmt::from_tokens(&tokens);

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
