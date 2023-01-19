#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("'{0}' not in variable table")]
    VarNotFound(String),

    #[error("'{0}' not valid variable name")]
    NotValidVar(String),

    #[error("Parser error, invalid string: '{0}'")]
    ParseInvalidString(String),
}
