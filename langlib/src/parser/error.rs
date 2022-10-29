use thiserror::Error;

use crate::lexer::token::{Token, TokenError};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ParserError {
    #[error("An invalid or incomplete let statement was encountered")]
    InvalidLetStatement,

    #[error("A token error has occured while parsing")]
    TokenError(#[from] TokenError),

    #[error("An incomplete term was encountered while parsing")]
    BadTerm,

    #[error("A token at an invalid index was tried to be accesed")]
    InvalidTokenIndex,

    #[error("An expected end to the stream of tokens was encountered")]
    UnexpectedEOF,

    #[error("Expected token \"{0:?}\"")]
    Expected(Token),
}