pub use self::rule::Rule;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

mod hidden;
mod rule;
mod visible;

pub struct YGGParser;
pub type RuleState<'a> = Box<::pest::ParserState<'a, Rule>>;
pub type RuleResult<'a> = Result<RuleState<'a>, RuleState<'a>>;

impl Parser<Rule> for YGGParser {
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        pest::state(input, |state| match rule {
            Rule::program => visible::program(state),
            Rule::statement => visible::statement(state),
            Rule::empty_statement => visible::empty_statement(state),
            Rule::eos => visible::eos(state),
            Rule::grammar_statement => visible::grammar_statement(state),
            Rule::grammar => visible::grammar(state),
            Rule::import_statement => visible::import_statement(state),
            Rule::import => visible::import(state),
            Rule::SYMBOL => visible::SYMBOL(state),
            Rule::EOI => visible::EOI(state),
        })
    }
}
