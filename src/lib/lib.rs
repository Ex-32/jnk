/// A library for parsing math expressions with stateful variables
mod ast;
pub mod context;
pub mod error;
mod parser;

pub use rug::Integer;
