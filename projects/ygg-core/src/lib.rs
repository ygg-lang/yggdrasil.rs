#![feature(try_blocks)]

// noinspection DuplicatedCode
pub mod codegen;
pub mod optimize;

mod utils;

pub use crate::utils::{parse_grammar, parse_grammar_raw};
pub use yggdrasil_error::{Failure, FileCache, Success, Url};
pub use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::GrammarRule,
};
