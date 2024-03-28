#![no_std]
// #![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc = include_str!("../readme.md")]

extern crate alloc;

use core::{fmt::Debug, hash::Hash};

pub use crate::{
    ast::YggdrasilNode,
    enhance::{stack::Stack, RegexCompiled},
    errors::{InvalidTag, YggdrasilError},
    iterators::{
        token_pair::{TokenPair, TokenTreeFilterRule, TokenTreeFilterTag},
        token_queue::TokenQueue,
        token_stream::TokenStream,
        token_tree::TokenTree,
        tokens::Tokens,
    },
    language::YggdrasilParser,
    parser_state::{state, Either, Lookahead, MatchDirection, State},
    position::Position,
    span::{merge_spans, Lines, LinesSpan, TextSpan},
    token::Token,
};
use daachorse::CharwiseDoubleArrayAhoCorasick;
#[cfg(not(debug_assertions))]
pub use regex_automata::dfa::regex::Regex;
#[cfg(debug_assertions)]
pub use regex_automata::meta::Regex;

pub type AhoCorasick<'i> = CharwiseDoubleArrayAhoCorasick<&'i str>;

pub mod errors;
mod iterators;
mod language;
mod parser_state;
mod position;

mod ast;

pub mod state;

mod enhance;
mod span;
mod token;

mod highlighters;
pub mod input;

/// Output result alias
pub type OutputResult<'i, R> = Result<TokenTree<'i, R>, YggdrasilError<R>>;

/// Define rules subject to Yggdrasil
pub trait YggdrasilRule: Clone + Debug + Eq + Hash + Ord {
    /// Nodes ignored in ast, such as spaces, carriage returns, comments, etc.
    fn is_ignore(&self) -> bool {
        false
    }

    fn is_leaf(&self) -> bool {
        false
    }

    /// To prevent violation of object safe, used to implement ord and eq.
    fn get_name(&self) -> &'static str {
        ""
    }
    /// To prevent violation of object safe, used to implement ord and eq.
    fn get_enum(&self) -> u32 {
        0
    }

    /// Get the style name from the rule
    fn get_style(&self) -> &'static str {
        ""
    }
}
