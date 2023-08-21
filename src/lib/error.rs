use crate::parser::Rule;
use rug::Integer;

#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("'{0}' not in variable table")]
    VarNotFound(String),

    #[error("'{0}' not valid variable name")]
    NotValidVar(String),

    #[error("parse error at: '{line}'\n{parse_failure}")]
    ParseError {
        line: String,
        parse_failure: Box<pest::error::Error<Rule>>,
    },

    #[error("can't raise base to '{0}' power, max 2^32-1")]
    ExponentOverflow(Integer),

    #[error("internal failure evaluating AST, please report this")]
    InternalAstFailure,
}
