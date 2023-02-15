#![allow(dead_code, unused_imports, non_camel_case_types)]

use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, Rule>>;
type Output<'i> = Result<Box<State<'i, Rule>>, Box<State<'i, Rule>>>;

#[derive(Default)]
pub struct Language {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    Json,
    String,
    Number,
    Null,
    WhiteSpace,
}

impl YggdrasilRule for Rule {
    fn all_rules() -> &'static [Self] {
        &[
            Self::Json,
            Self::String,
            Self::Number,
            Self::Null,
            Self::WhiteSpace,
        ]
    }

    fn is_ignore(&self) -> bool {
        false
    }
}

impl YggdrasilParser for Language {
    type Rule = Rule;
    fn parse(input: &str, rule: Rule) -> OutputResult<Rule> {
        state(input, |state| match rule {
            Rule::Json => parse_json(state),
            Rule::String => parse_string(state),
            Rule::Number => parse_number(state),
            Rule::Null => parse_null(state),
            Rule::WhiteSpace => parse_white_space(state),
        })
    }
}
#[inline]
fn parse_json(state: Input) -> Output {
    state.rule(Rule::Json, |s| {
        parse_Choice(s)
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(Rule::String, |s| {
        parse_Choice(s)
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Rule::Number, |s| {
        parse_Choice(s)
    })
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Rule::Null, |s| {
        s.match_string("null")
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Rule::WhiteSpace, |s| {
        parse_Choice(s)
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn parse_ignore(state: Input) -> Output {
    Ok(state)
}