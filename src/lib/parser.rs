use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "lib/grammar/math.pest"]
pub(crate) struct MathParser;
