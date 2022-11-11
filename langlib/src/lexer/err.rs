use std::num::IntErrorKind;

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum LexerError {
    #[error("Failed to parse int")]
    IntError(IntErrorKind),
    #[error("Invalid token encountered")]
    InvalidChar(char),
    #[error("Unexpected EOF encountered")]
    UnexpectedEOF,
    #[error("Expected '{0}'")]
    Expected(char),
}
