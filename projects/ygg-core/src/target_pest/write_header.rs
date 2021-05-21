use super::*;

pub fn write_header(f: &mut impl Write, parser_name: &str) -> std::fmt::Result {
    writeln!(
        f,
        r#"
#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

pub use self::rule::Rule;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Atomicity::*;
use pest::Parser;
use pest::ParserState;

mod parse;
mod rule;

pub struct {name};
pub type RuleState<'a> = Box<ParserState<'a, Rule>>;
pub type RuleResult<'a> = Result<RuleState<'a>, RuleState<'a>>;
"#,
        name = parser_name
    )
}
