
#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("'{0}' not in variable table")]
    VarNotFound(String),

    #[error("'{0}' not valid variable name")]
    NotValidVar(String),
}
