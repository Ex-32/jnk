use std::collections::HashMap;

use crate::ast::{Node, Operator};
use crate::Integer;
use crate::{
    error::Error,
    parser::{MathParser, Rule},
};
use pest::Parser;
use rug::ops::Pow;

/// A math environment to evaluate expressions.
///
/// Create a blank context by calling `MathContext::new()`, add variables with
/// `var_set()`, and evaluate expressions with `eval()`
///
/// # Examples
/// ```
/// use jnk::context::MathContext;
///
/// let mut context = MathContext::new();
/// context.var_set("myVar".to_string(), 42.into()).unwrap();
///
/// assert_eq!(
///     *context.var_get("myVar").unwrap(),
///     jnk::Integer::from(42)
/// );
/// assert_eq!(
///     context.eval("myVar / 2").unwrap().value,
///     jnk::Integer::from(21)
/// );/
/// assert_eq!(
///     context.eval("2 ^ 32 - 1").unwrap().value,
///     jnk::Integer::from(u32::MAX)
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

    /// Checks if `name` is a valid context variable name (ascii alphanumeric,
    /// and starts with a letter); used internally in [`Self::var_set()`].
    #[inline]
    pub fn var_valid(name: &str) -> bool {
        (name.starts_with(|x: char| x.is_ascii_alphabetic())
            && name.chars().all(|x| x.is_ascii_alphanumeric()))
            || name == "_"
    }

    /// Adds a variable to the context so that it can be used in expressions,
    /// if the variable already exists its value is overwritten. Returns
    /// [`Error::NotValidVar`] if the variable name is invalid; variable names
    /// must be ascii alphanumeric, and begin with a letter OR be the special
    /// variable "`_`" which always refers to the last returned value (so values
    /// assigned to it are effectively disregarded).
    pub fn var_set(&mut self, name: String, value: Integer) -> Result<(), Error> {
        if Self::var_valid(&name) {
            self.var_tab.insert(name, value);
            Ok(())
        } else if name == "_" {
            Ok(())
        } else {
            Err(Error::NotValidVar(name))
        }
    }

    /// Retrieves a variable from the context, returns `None` if the variable
    /// name doesn't exist in the context (this includes invalid names). the
    /// specaial variable "`_`" returns the last returned value, although this
    /// is primairly to match the syntax of [`MathContext::eval()`], it's better
    /// to just call [`MathContext::last()`] directly.
    pub fn var_get(&self, name: &str) -> Option<&Integer> {
        if name == "_" {
            Some(self.last())
        } else {
            self.var_tab.get(name)
        }
    }

    /// Evaluate a math expression, this will update the last result value,
    /// as well as store the result into the left hand variable (if it exists)
    /// returns the value and if it was stored.
    pub fn eval(&mut self, expr: &str) -> Result<Eval, Error> {
        let mut res = self.eval_internal(expr)?;
        if let Some(lhs) = &res.var {
            if lhs == "_" {
                res.var = None
            } else {
                self.var_set(lhs.to_owned(), res.value.to_owned())?;
            }
        }
        self.last = res.value.clone();
        Ok(res)
    }

    /// Evaluate a math expression, and disregard the result, this will **not**
    /// update the last result value, and the left hand variable will be
    /// disregarded (if it exists)
    pub fn eval_disregard(&self, expr: &str) -> Result<Integer, Error> {
        let res = self.eval_internal(expr)?;
        Ok(res.value)
    }

    fn eval_internal(&self, expr: &str) -> Result<Eval, Error> {
        let mut pairs = match MathParser::parse(Rule::Main, expr) {
            Ok(x) => x,
            Err(e) => {
                return Err(Error::ParseError {
                    line: expr.to_owned(),
                    parse_failure: Box::new(e),
                })
            }
        };

        let ast = crate::ast::create_ast(
            pairs
                .next()
                .ok_or_else(|| unreachable!("Rule::Main can't be empty"))?,
        );

        if let Node::Main(lhs, mut expr) = ast {
            let var = lhs.map(|x| match *x {
                Node::Lhs(x) => x,
                _ => unreachable!(),
            });
            let value = self.eval_ast(&mut expr)?;
            Ok(Eval { var, value })
        } else {
            unreachable!("AST generated from Rule::Main must have root level Main node")
        }
    }

    fn eval_ast(&self, ast: &mut Node) -> Result<Integer, Error> {
        match ast {
            Node::Main(_, expr) => self.eval_ast(expr),
            Node::Variable(var) => match self.var_get(var) {
                Some(x) => Ok(x.clone()),
                None => Err(Error::VarNotFound(var.clone())),
            },
            Node::Lhs(_) => Err(Error::InternalAstFailure),
            Node::Operator(_) => Err(Error::InternalAstFailure),
            Node::Parenthetical(inner) => self.eval_ast(inner),
            Node::Negation(inner) => Ok(-(self.eval_ast(inner)?)),
            Node::Literal(x) => Ok(x.clone()),
            Node::Expression(line) => {
                // Parenthesizes
                // clippy is telling me to just use a standard iterator loop,
                // but then i can't reassign the value back into `line` (at
                // least not any way i found) so i just suppresed the warning
                #[allow(clippy::needless_range_loop)]
                for i in 0..line.len() {
                    if let Some(Node::Parenthetical(_)) = line[i] {
                        let mut node = line[i].take().ok_or(Error::InternalAstFailure)?;
                        line[i] = Some(Node::Literal(self.eval_ast(&mut node)?));
                    } else if let Some(Node::Negation(_)) = line[i] {
                        let mut node = line[i].take().ok_or(Error::InternalAstFailure)?;
                        line[i] = Some(Node::Literal(self.eval_ast(&mut node)?))
                    }
                }

                // Exponents
                for i in 0..line.len() {
                    if let Some(Node::Operator(Operator::Exponent)) = line[i] {
                        let lhs = self.eval_ast(&mut node_left(line, i)?)?;
                        let rhs = self.eval_ast(&mut node_right(line, i)?)?;

                        let val: Integer;
                        if rhs < 0 {
                            match rhs.as_neg().to_u32() {
                                Some(x) => val = Integer::from(1) / lhs.pow(x),
                                None => return Err(Error::ExponentOverflow(rhs)),
                            }
                        } else {
                            match rhs.to_u32() {
                                Some(x) => val = lhs.pow(x),
                                None => return Err(Error::ExponentOverflow(rhs)),
                            }
                        }
                        line[i] = Some(Node::Literal(val));
                    }
                }

                // Multiplication & Division
                for i in 0..line.len() {
                    if let Some(Node::Operator(op)) = line[i] {
                        if let Operator::Multiplication = op {
                            let lhs = self.eval_ast(&mut node_left(line, i)?)?;
                            let rhs = self.eval_ast(&mut node_right(line, i)?)?;
                            line[i] = Some(Node::Literal(lhs * rhs));
                        } else if let Operator::Division = op {
                            let lhs = self.eval_ast(&mut node_left(line, i)?)?;
                            let rhs = self.eval_ast(&mut node_right(line, i)?)?;
                            line[i] = Some(Node::Literal(lhs / rhs));
                        }
                    }
                }

                // Addition & Subtraction
                for i in 0..line.len() {
                    if let Some(Node::Operator(op)) = line[i] {
                        if let Operator::Addition = op {
                            let lhs = self.eval_ast(&mut node_left(line, i)?)?;
                            let rhs = self.eval_ast(&mut node_right(line, i)?)?;
                            line[i] = Some(Node::Literal(lhs + rhs));
                        } else if let Operator::Subtraction = op {
                            let lhs = self.eval_ast(&mut node_left(line, i)?)?;
                            let rhs = self.eval_ast(&mut node_right(line, i)?)?;
                            line[i] = Some(Node::Literal(lhs - rhs));
                        }
                    }
                }

                // as mentioned above i couldn't get a normal iterator loop to
                // work properly with the borrow checker, so I'm just telling
                // clippy to stfu
                #[allow(clippy::needless_range_loop)]
                for i in 0..line.len() {
                    if let Some(Node::Variable(x)) = &line[i] {
                        line[i] = Some(Node::Literal(
                            self.eval_ast(&mut Node::Variable(x.to_owned()))?,
                        ));
                    }
                }

                let result = line
                    .iter_mut()
                    .filter_map(|x| x.as_ref())
                    .collect::<Vec<_>>();

                if result.len() == 1 {
                    if let Node::Literal(x) = result[0] {
                        Ok(x.clone())
                    } else {
                        Err(Error::InternalAstFailure)
                    }
                } else {
                    Err(Error::InternalAstFailure)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Eval {
    pub value: Integer,
    pub var: Option<String>,
}

fn node_left<T>(line: &mut [Option<T>], mut i: usize) -> Result<T, Error> {
    Ok(loop {
        if i == 0 {
            return Err(Error::InternalAstFailure);
        }
        i -= 1;
        if line[i].is_some() {
            break line[i].take().ok_or(Error::InternalAstFailure)?;
        }
    })
}

fn node_right<T>(line: &mut [Option<T>], mut i: usize) -> Result<T, Error> {
    Ok(loop {
        if i == line.len() {
            return Err(Error::InternalAstFailure);
        }
        i += 1;
        if line[i].is_some() {
            break line[i].take().ok_or(Error::InternalAstFailure)?;
        }
    })
}
