#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Atomicity::*;
use pest::Parser;
use pest::ParserState;

mod parse;
mod rule;

pub struct YGGParser;
pub type RuleState<'a> = Box<ParserState<'a, Rule>>;
pub type RuleResult<'a> = Result<RuleState<'a>, RuleState<'a>>;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program_entry_point,
    program,
    statement,
    empty_statement,
    eos,
    grammar_statement,
    grammar,
    import_statement,
    import,
    SYMBOL,
}

impl Parser<Rule> for YGGParser {
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        pest::state(input, |state| match rule {
            Rule::program_entry_point => {
                unimplemented!()
            }
            Rule::program => parse::program(state),
            Rule::statement => parse::statement(state),
            Rule::empty_statement => parse::empty_statement(state),
            Rule::eos => parse::eos(state, false),
            Rule::grammar_statement => parse::grammar_statement(state),
            Rule::grammar => parse::grammar(state),
            Rule::import_statement => parse::import_statement(state),
            Rule::import => parse::import(state),
            Rule::SYMBOL => parse::SYMBOL(state),
            Rule::EOI => parse::EOI(state),
        })
    }
}
