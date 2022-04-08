use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    ops::{Range, RangeInclusive},
};

use num::BigInt;
use serde::{Deserialize, Serialize};

use character_set::CharacterSet;

use crate::{
    data::charset::{char_range_display, char_set_display, string_display},
    *,
};

pub mod builtin;
pub mod charset;
pub mod rule_ref;
pub mod symbol;

//
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum DataKind {
    Integer(BigInt),
    String(String),
    CharacterAny,
    Character(char),
    CharacterBuiltin(String),
    CharacterRange(RangeInclusive<char>),
    CharacterSet(CharacterSet),
}

impl Display for DataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Integer(i) => write!(f, "{}", i),
            DataKind::String(s) => string_display(s, f),
            DataKind::CharacterAny => write!(f, "ANY"),
            DataKind::Character(c) => write!(f, "{:?}", c),
            DataKind::CharacterRange(range) => char_range_display(range, f),
            DataKind::CharacterBuiltin(set) => write!(f, "{}", set),
            DataKind::CharacterSet(set) => char_set_display(set, f),
        }
    }
}

impl DataKind {}
