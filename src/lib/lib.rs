/// A library for parsing math expressions with stateful variables
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
pub mod context;
pub mod error;
mod parser;

pub use rug::Integer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn deny_bad_name() {
        let mut ctx = context::MathContext::new();
        ctx.var_set("%!@*&$".to_owned(), Integer::from(42)).unwrap();
    }
}
