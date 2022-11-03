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
    InvalidLetStatement,

    #[error("A token error has occured while parsing")]
    TokenError(#[from] TokenError),

    #[error("A statement error has occured while parsing")]
    StmtErr(#[from] StmtErr),

    #[error("An expression error has occured while parsing")]
    ExprError(#[from] ExprError),

    #[error("An incomplete term was encountered while parsing")]
    BadTerm,

    #[error("A token at an invalid index was tried to be accesed")]
    InvalidTokenIndex,

    #[error("An expected end to the stream of tokens was encountered")]
    UnexpectedEOF,

    #[error("Expected token \"{0:?}\"")]
    Expected(Token),

    #[error("Expected expression")]
    ExpectedExpr,

    #[error("Failed to parse statement")]
    BadStatement,

    #[error("A case where recursion in the parser was provoked has been encountered.")]
    RecursionDetected,

    #[error("An invalid comparision was attempted to be made.")]
    InvalidComparision,
}
