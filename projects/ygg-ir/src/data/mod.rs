mod builtin;
mod charset;
mod regex_category;
mod rule_ref;
mod symbol;

pub use self::{
    regex_category::YggdrasilRegex,
    rule_ref::RuleReference,
    symbol::{Symbol, SymbolAlias},
};
use crate::{
    data::charset::{char_range_display, char_set_display, string_display},
    nodes::{ExpressionKind, YggdrasilExpression},
    rule::YggdrasilIdentifier,
};
use character_set::CharacterSet;
use num::BigInt;
use regex_automata::dfa::{dense::BuildError, regex::Regex};
use serde::{de::Visitor, ser::SerializeTupleStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    fmt::{Debug, Display, Formatter, Write},
    hash::{BuildHasher, Hash, Hasher},
    ops::{Range, RangeInclusive},
    str::FromStr,
};

//
#[derive(Debug, Clone)]
pub enum DataKind {
    Integer(BigInt),
    CharacterRange(RangeInclusive<char>),
    CharacterBuiltin(String),
    CharacterFused(CharacterSet),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct YggdrasilText {
    pub text: String,
    pub insensitive: bool,
    pub range: Range<usize>,
}

impl From<YggdrasilText> for YggdrasilExpression {
    fn from(value: YggdrasilText) -> Self {
        ExpressionKind::Text(value).into()
    }
}

impl Display for DataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Integer(v) => write!(f, "{}", v),
            DataKind::String(v) => string_display(v, f),
            DataKind::Character(c) => write!(f, "{:?}", c),
            DataKind::CharacterRange(range) => char_range_display(range, f),
            DataKind::CharacterBuiltin(set) => write!(f, "{}", set),
            DataKind::CharacterFused(set) => char_set_display(set, f),
        }
    }
}

impl From<DataKind> for YggdrasilExpression {
    fn from(value: DataKind) -> Self {
        ExpressionKind::Data(Box::new(value)).into()
    }
}

impl DataKind {}
