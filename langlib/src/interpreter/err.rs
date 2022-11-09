#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum RuntimeErr {
    #[error("Variable \"{0}\" already exists.")]
    VarRedefine(String),
    #[error("Variable \"{0}\" does not exist.")]
    UndefinedVar(String),
}