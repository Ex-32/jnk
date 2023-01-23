use rug::Integer;

#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("'{0}' not in variable table")]
    VarNotFound(String),

    #[error("'{0}' not valid variable name")]
    NotValidVar(String),

    #[error("Parser error, invalid string: '{0}'")]
    ParseInvalidString(String),

    #[error("can't raise base to '{0}' power, max 2^32-1")]
    ExponentOverflow(Integer),

    #[error("Internal failure evaluating AST, please report this")]
    InternalAstFailure,
}
