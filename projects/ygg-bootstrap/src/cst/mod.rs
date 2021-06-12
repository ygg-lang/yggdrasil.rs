#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

// #[rustfmt::skip]
mod parse;
mod rule;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Atomicity;
use pest::Parser;
use pest::ParserState;
use yggdrasil_cst_shared::{ignore_terms, match_charset, tag_branch, tag_node};

pub use self::rule::Rule;

pub type RuleState<'a> = Box<ParserState<'a, Rule>>;
pub type RuleResult<'a> = Result<RuleState<'a>, RuleState<'a>>;

pub struct CSTBuilder {}
