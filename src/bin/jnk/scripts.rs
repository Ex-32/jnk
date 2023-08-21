use std::{fs, path::Path};

use crate::parser::{FileParser, Rule};
use color_eyre::eyre::{eyre, Context, Result};
use pest::{iterators::Pair, Parser};

#[derive(Debug, Clone)]
pub(crate) struct MathScript {
    lines: Vec<String>,
}

impl MathScript {
    pub fn from_file(file: &Path) -> Result<Self> {
        Self::from_str(fs::read_to_string(file).wrap_err("unable to read script file")?)
            .wrap_err("unable to parse script file")
    }

    pub fn from_str(mut script: String) -> Result<Self> {
        if !script.ends_with('\n') {
            script.push('\n');
        }

        Ok(Self::from_pairs(
            match FileParser::parse(Rule::Main, &script)
                .wrap_err("parser failure:")?
                .next()
            {
                Some(x) => x,
                None => return Ok(Self { lines: Vec::new() }),
            },
        ))
    }

    #[inline]
    fn from_pairs(pair: Pair<Rule>) -> Self {
        Self {
            lines: Self::from_pairs_internal(pair, Vec::new()),
        }
    }

    fn from_pairs_internal(pair: Pair<Rule>, mut vec: Vec<String>) -> Vec<String> {
        match pair.as_rule() {
            Rule::Main => {
                for p in pair.into_inner() {
                    vec = Self::from_pairs_internal(p, vec);
                }
            }
            Rule::Line => vec.push(pair.as_str().trim().to_owned()),
            Rule::EOI => (),
            Rule::COMMENT => unreachable!("Non-Silent Silent Rule (COMMENT)"),
            Rule::WHITESPACE => unreachable!("Non-Silent Silent Rule (WHITESPACE)"),
        };
        vec
    }

    pub fn eval(&self) -> Result<()> {
        let mut ctx = jnk::context::MathContext::new();
        self.lines.iter().try_for_each(|x| match ctx.eval(x) {
            Ok(x) => {
                if x.var.is_none() {
                    println!("{}", x.value)
                }
                Ok(())
            }
            Err(e) => Err(eyre!(e)),
        })
    }
}
