/// A library for parsing math expressions with stateful variables


extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
mod error;

use std::collections::HashMap;

pub use error::Error;
pub use rug::Integer;

/// A math environment to evaluate expressions.
///
/// Create a blank context by calling `MathContext::new()`, add variables with
/// `var_set()`, and evaluate expressions with `eval()`
///
/// # Examples
///
/// ```
/// use jnk;
/// let context = jnk::MathContext::new();
/// context.add_var("myVar", 42);
/// assert_eq!(context.get_var("myVar"), 42);
/// ```
///
#[derive(Debug, Clone, Default)]
pub struct MathContext {
    var_tab: HashMap<String, Integer>,
    last: Integer,
}

impl MathContext {
    /// Creates a new math context, with a blank variable table
    #[inline]
    pub fn new() -> Self { MathContext::default() }

    /// Returns the result of the last (non-discarded) expression evaluated
    #[inline]
    pub fn last(&self) -> &Integer { &self.last }

    /// Adds a variable to the context so that it can be used in expressions,
    /// if the variable already exists its value is overwritten. Returns a
    /// `NotValidVar` error if the variable name is invalid; variable names must
    /// be ascii alphanumeric, and begin with a letter.
    pub fn var_set(&mut self, name: String, value: Integer) -> Result<(), Error> {
        if name.starts_with(|x: char| x.is_ascii_alphabetic()) &&
           name.chars().all(|x: char| x.is_ascii_alphanumeric()) {
            self.var_tab.insert(name, value);
            return Ok(());
        } else {
            return Err(Error::NotValidVar(name));
        }
    }

    /// Adds a variable to the context so that it can be used in expressions,
    /// if the variable already exists its value is overwritten. Does not check
    /// if name is variable name is valid, **the caller must ensure that the
    /// variable name is ascii alphanumeric, and begins with a letter**.
    #[inline]
    pub unsafe fn var_set_unchecked(&mut self, name: String, value: Integer) {
        self.var_tab.insert(name, value);
    }

    /// Retrieves a variable from the context, returns `None` if the variable
    /// name doesn't exist in the context (this includes invalid names).
    #[inline]
    pub fn var_get<'a>(&'a self, name: &str) -> Option<&'a Integer> {
        self.var_tab.get(name)
    }

    /// Evaluate a math expression, this will update the last result value,
    /// as well as store the result into the left hand variable (if it exists)
    pub fn eval(&mut self, expr: &str) -> Result<Integer, Error> {
        todo!()
    }

    /// Evaluate a math expression, and disregard the result, this will **not**
    /// update the last result value, and the left hand variable will be
    /// disregarded (if it exists)
    pub fn eval_disregard(&self, expr: &str) -> Result<Integer, Error> {
        todo!()
    }

    fn parse_expr(&self, expr: &str) -> Result<ExprResult, Error> {

        todo!()
    }
}

#[derive(Debug, Clone)]
struct ExprResult {
    value: Integer,
    lhs: Option<String>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
