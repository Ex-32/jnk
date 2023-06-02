/// A library for parsing math expressions with stateful variables
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
pub mod context;
pub mod error;
mod parser;

pub use rug::Integer;
