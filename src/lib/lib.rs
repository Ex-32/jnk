/// A library for parsing math expressions with stateful variables
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod error;
mod parser;

use std::collections::HashMap;

use crate::{
    error::Error,
    parser::{MathParser, Rule},
};
use ast::{Node, Operator};
use pest::Parser;
pub use rug::Integer;

/// A math environment to evaluate expressions.
///
/// Create a blank context by calling `MathContext::new()`, add variables with
/// `var_set()`, and evaluate expressions with `eval()`
///
/// # Examples
/// ```
/// let mut context = jnk::MathContext::new();
/// context.var_set(
///     String::from("myVar"),
///     jnk::Integer::from(42)
/// ).unwrap();
/// assert_eq!(
///     *context.var_get("myVar").unwrap(),
///     jnk::Integer::from(42)
/// );
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
    pub fn new() -> Self {
        MathContext::default()
    }

    /// Returns the result of the last (non-discarded) expression evaluated (initially zero)
    #[inline]
    pub fn last(&self) -> &Integer {
        &self.last
    }

    /// Adds a variable to the context so that it can be used in expressions,
    /// if the variable already exists its value is overwritten. Returns a
    /// `NotValidVar` error if the variable name is invalid; variable names must
    /// be ascii alphanumeric, and begin with a letter.
    pub fn var_set(&mut self, name: String, value: Integer) -> Result<(), Error> {
        if name.starts_with(|x: char| x.is_ascii_alphabetic())
            && name.chars().all(|x: char| x.is_ascii_alphanumeric())
        {
            self.var_tab.insert(name, value);
            Ok(())
        } else {
            Err(Error::NotValidVar(name))
        }
    }

    /// Adds a variable to the context so that it can be used in expressions,
    /// if the variable already exists its value is overwritten. Does not check
    /// if name is variable name is valid!
    ///
    /// # Safety
    /// the caller must ensure that the variable name is ascii alphanumeric, and
    /// begins with a letter
    ///
    #[inline]
    pub unsafe fn var_set_unchecked(&mut self, name: String, value: Integer) {
        self.var_tab.insert(name, value);
    }

    /// Retrieves a variable from the context, returns `None` if the variable
    /// name doesn't exist in the context (this includes invalid names).
    #[inline]
    pub fn var_get(&self, name: &str) -> Option<&Integer> {
        self.var_tab.get(name)
    }

    /// Evaluate a math expression, this will update the last result value,
    /// as well as store the result into the left hand variable (if it exists)
    pub fn eval(&mut self, expr: &str) -> Result<Integer, Error> {
        let res = self.eval_internal(expr)?;
        if let Some(lhs) = res.lhs {
            self.var_set(lhs, res.value.clone())?;
        }
        self.last = res.value.clone();
        Ok(res.value)
    }

    /// Evaluate a math expression, and disregard the result, this will **not**
    /// update the last result value, and the left hand variable will be
    /// disregarded (if it exists)
    pub fn eval_disregard(&self, expr: &str) -> Result<Integer, Error> {
        let res = self.eval_internal(expr)?;
        Ok(res.value)
    }

    fn eval_internal(&self, expr: &str) -> Result<ExprResult, Error> {
        let mut pairs = match MathParser::parse(Rule::Main, expr) {
            Ok(x) => x,
            Err(_) => return Err(Error::ParseInvalidString(expr.to_owned())),
        };

        let mut ast = ast::create_ast(pairs.next().unwrap());

        println!("{:#?}", ast);

        let val = self.eval_ast(&ast);

        todo!()
    }

    fn eval_ast(&self, ast: &Node) -> Result<Integer, Error> {
        match ast {
            Node::Main(_, expr) => self.eval_ast(expr),
            Node::Variable(_) => todo!(),
            Node::Lhs(_) => todo!(),
            Node::Operator(_) => todo!(),
            Node::Parenthetical(_) => todo!(),
            Node::Literal(_) => todo!(),
            Node::Expression(line) => {
                // let mut line = line.clone();
                // Parenthesizes
                for node in line {
                    if let Node::Parenthetical(inner) = node {
                        return self.eval_ast(&inner);
                    }
                }

                // Exponenets
                for (i, node) in line.iter().enumerate()  {
                    if let Node::Operator(x) = node {
                        if let Operator::Exponent = x {
                            let lhs = self.eval_ast(&line[i-1])?;
                            let mut rhs = self.eval_ast(&line[i+1])?;
                            let mut a = lhs.clone();

                            // line.remove(i-1);
                            // line.remove(i+1);

                            while rhs.clone() > 1 {
                                a = Integer::from(&a * &lhs);
                                rhs = rhs.clone() - 1;
                            }
                        }
                    }
                }


                unreachable!()
            },
        }
    }
}

#[derive(Debug, Clone)]
struct ExprResult {
    value: Integer,
    lhs: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn deny_bad_name() {
        let mut ctx = MathContext::new();
        ctx.var_set("%!@*&$".to_owned(), Integer::from(42)).unwrap();
    }
}
