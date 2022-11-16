use thiserror::Error;

use crate::{
    expr::ExprError,
    lexer::token::{Token, TokenError},
    stmt::StmtErr,
};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
/// Error enum for the `Parser` struct.
pub enum ParserError {
    #[error("An invalid or incomplete let statement was encountered")]
    InvalidLetStatement(usize),

    #[error("A token error has occured while parsing")]
    TokenError(#[from] TokenError),

    #[error("A statement error has occured while parsing")]
    StmtErr(#[from] StmtErr),

    #[error("An expression error has occured while parsing")]
    ExprError(#[from] ExprError),

    #[error("An incomplete term was encountered while parsing.")]
    BadTerm(usize),

    #[error("A token at an invalid index was tried to be accesed")]
    InvalidTokenIndex(usize),

    #[error("An expected end to the stream of tokens was encountered")]
    UnexpectedEOF,

    #[error("Expected token \"{0:?}\"")]
    Expected(Token, usize),

    #[error("Expected expression")]
    ExpectedExpr(usize),

    #[error("Failed to parse statement")]
    BadStatement(usize),

    #[error("An internal error within the parser occured.")]
    EmptyMatch(usize),

    #[error("An invalid comparision was attempted to be made.")]
    InvalidComparision(usize),

    #[error("An unexpected token was found.")]
    UnexpectedToken(Token, usize),

    #[error("Only a maximum capacity of 254 arguments is supported.")]
    TooManyArgs(usize),

}