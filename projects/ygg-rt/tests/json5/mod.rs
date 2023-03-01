#![allow(dead_code, unused_imports, non_camel_case_types)]
#![doc = include_str!("readme.md")]

mod lexer;
mod parse_ast;
mod parse_cst;

use std::sync::OnceLock;
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, Json5Rule>>;
type Output<'i> = Result<Box<State<'i, Json5Rule>>, Box<State<'i, Json5Rule>>>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Json5Language {}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Json5Rule {
    Value,
    Object,
    ObjectPair,
    Array,
    String,
    StringEscaped,
    Number,
    Boolean,
    Null,
    Identifier,
    WhiteSpace,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for Json5Rule {
    fn all_rules() -> &'static [Self] {
        &[
            Self::Value,
            Self::Object,
            Self::ObjectPair,
            Self::Array,
            Self::String,
            Self::StringEscaped,
            Self::Number,
            Self::Boolean,
            Self::Null,
            Self::Identifier,
            Self::WhiteSpace,
        ]
    }

    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex | Self::WhiteSpace)
    }
}
