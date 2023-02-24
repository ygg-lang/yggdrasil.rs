#![no_std]
#![warn(missing_docs, unused_qualifications)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc = include_str!("../readme.md")]

extern crate alloc;

pub use self::regex_proxy::RegexCompiled;
pub use crate::{
    parser::YggdrasilParser,
    parser_state::{state, Either, Lookahead, MatchDir, State},
    position::Position,
    span::{merge_spans, Lines, LinesSpan, TextSpan},
    stack::Stack,
    token::Token,
};
use core::{fmt::Debug, hash::Hash};
pub use regex_automata::dfa::regex::Regex;
pub mod errors;
pub mod iterators;

mod macros;
mod parser;
mod parser_state;
mod position;

pub mod pratt_parser;
#[deprecated(
    since = "2.4.0",
    note = "Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release."
)]
pub mod prec_climber;
mod regex_proxy;
mod span;
mod stack;
mod token;
pub use crate::errors::OutputResult;
#[doc(hidden)]
pub mod unicode;

/// Define rules subject to Yggdrasil
pub trait YggdrasilRule: Copy + Debug + Eq + Hash + Ord {
    /// Go through all the rules
    fn all_rules() -> &'static [Self];
    /// Nodes ignored in ast, such as spaces, carriage returns, comments, etc.
    fn is_ignore(&self) -> bool;
}
