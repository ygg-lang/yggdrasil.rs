#![no_std]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc = include_str!("../readme.md")]

extern crate alloc;

mod errors;
mod iterators;
mod language;
mod parser_state;
mod position;

mod ast;

mod enhance;
pub mod pratt_parser;
#[deprecated(
    since = "2.4.0",
    note = "Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release."
)]
pub mod prec_climber;
mod span;
mod token;

mod highlighters;

pub use crate::{
    ast::YggdrasilNode,
    enhance::{stack::Stack, RegexCompiled},
    errors::YggdrasilError,
    iterators::TokenTree,
    language::YggdrasilParser,
    parser_state::{state, Either, Lookahead, MatchDir, State},
    position::Position,
    span::{merge_spans, Lines, LinesSpan, TextSpan},
    token::Token,
};
use core::{fmt::Debug, hash::Hash};
pub use regex_automata::dfa::regex::Regex;

/// Output result alias
pub type OutputResult<'i, R> = Result<TokenTree<'i, R>, YggdrasilError<R>>;

/// Define rules subject to Yggdrasil
pub trait YggdrasilRule: Clone + Debug + Eq + Hash + Ord {
    /// Nodes ignored in ast, such as spaces, carriage returns, comments, etc.
    fn is_ignore(&self) -> bool {
        false
    }
    /// Get the style name from the rule
    fn get_style(&self) -> &'static str {
        ""
    }
}
