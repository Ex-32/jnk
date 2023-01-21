use std::str::FromStr;

use crate::{parser::Rule, error::Error};
use rug::Integer;

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Main(Option<Box<Node>>, Box<Node>),
    Variable(String),
    Lhs(String),
    Operator(Operator),
    Parenthetical(Box<Node>),
    Literal(Integer),
    Expression(Vec<Node>),
}

#[derive(Debug, Clone)]
pub(crate) enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
}

pub(crate) fn create_ast(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Variable => Node::Variable(pair.as_str().to_owned()),
        Rule::Lhs => Node::Lhs(pair.as_str().to_owned()),
        Rule::Operator => Node::Operator(match pair.as_str() {
            "+" => Operator::Addition,
            "-" => Operator::Subtraction,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "^" => Operator::Exponent,
            _ => unreachable!("Not an operator string"),
        }),
        Rule::Parenthetical => {
            Node::Parenthetical(Box::new(create_ast(pair.into_inner().next().unwrap())))
        }
        Rule::Literal => Node::Literal(Integer::from_str(pair.as_str()).unwrap()),
        Rule::Value => create_ast(pair.into_inner().next().unwrap()),
        Rule::Expression => Node::Expression(pair.into_inner().map(create_ast).collect()),
        Rule::Main => {
            let mut pairs = pair.into_inner();
            let val = pairs.next().unwrap();
            if val.as_rule() == Rule::Lhs {
                Node::Main(
                    Some(Box::new(create_ast(val))),
                    Box::new(create_ast(pairs.next().unwrap())),
                )
            } else {
                Node::Main(None, Box::new(create_ast(val)))
            }
        }
        Rule::EOI => unreachable!("Non-Silent Silent Rule (EOI)"),
        Rule::WHITESPACE => unreachable!("Non-Silent Silent Rule (WHITESPACE)"),
    }
}
