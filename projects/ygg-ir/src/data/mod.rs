mod builtin;
mod charset;
mod rule_ref;
mod serder;
mod symbol;

pub use self::{
    rule_ref::RuleReference,
    symbol::{Symbol, SymbolAlias},
};
use crate::{
    data::charset::{char_range_display, char_set_display, string_display},
    nodes::{ExpressionKind, ExpressionNode},
};

use crate::rule::YggdrasilIdentifier;
use character_set::CharacterSet;
use num::BigInt;
use regex_automata::dfa::regex::Regex;
use serde::{
    de::{Error, Visitor},
    ser::SerializeTupleStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    collections::hash_map::RandomState,
    fmt::{Debug, Display, Formatter, Write},
    hash::{BuildHasher, Hash, Hasher},
    ops::{Range, RangeInclusive},
};

//
#[derive(Debug, Clone)]
pub enum DataKind {
    Ignored,
    Boolean(bool),
    Integer(BigInt),
    String(String),
    StringFused(Regex),
    CharacterAny,
    Character(char),
    CharacterRange(RangeInclusive<char>),
    CharacterBuiltin(String),
    CharacterFused(CharacterSet),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct YggdrasilText {
    pub text: String,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RegularExpression {
    pub text: String,
    pub span: Range<usize>,
}

impl Display for DataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Ignored => write!(f, "IGNORED"),
            DataKind::Boolean(v) => write!(f, "{}", v),
            DataKind::Integer(v) => write!(f, "{}", v),
            DataKind::String(v) => string_display(v, f),
            DataKind::StringFused(_) => write!(f, "FUSED_STRING"),
            DataKind::CharacterAny => write!(f, "ANY"),
            DataKind::Character(c) => write!(f, "{:?}", c),
            DataKind::CharacterRange(range) => char_range_display(range, f),
            DataKind::CharacterBuiltin(set) => write!(f, "{}", set),
            DataKind::CharacterFused(set) => char_set_display(set, f),
        }
    }
}

impl From<DataKind> for ExpressionNode {
    fn from(value: DataKind) -> Self {
        Self { tag: String::new(), kind: ExpressionKind::Data(Box::new(value)) }
    }
}

impl DataKind {}
