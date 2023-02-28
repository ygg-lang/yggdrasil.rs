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
    nodes::{ExpressionKind, YggdrasilExpression},
    rule::YggdrasilIdentifier,
};
use character_set::CharacterSet;
use regex_automata::dfa::{dense::BuildError, regex::Regex};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
    ops::{Range, RangeInclusive},
    str::FromStr,
};

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
