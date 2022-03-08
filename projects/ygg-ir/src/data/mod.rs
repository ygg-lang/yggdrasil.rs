use std::{
    fmt::{Display, Formatter, Write},
    hash::Hash,
    ops::Range,
};

use num::BigInt;

use character_set::CharacterSet;

use crate::{data::charset::string_display, *};

pub mod builtin;
pub mod charset;
pub mod rule_ref;
pub mod symbol;

//
// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum DataKind {
    Integer(BigInt),
    String(String),
    Rule(RuleReference),
    CharacterAny,
    Character(char),
    CharacterRange(Range<char>),
    CharacterSet(CharacterSet),
}

impl Display for DataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Integer(i) => {
                write!(f, "{}", i)
            }
            DataKind::String(s) => string_display(s, f)?,
            DataKind::Rule(_) => {
                todo!()
            }
            DataKind::CharacterAny => {
                todo!()
            }
            DataKind::Character { .. } => {
                todo!()
            }
            DataKind::CharacterRange { .. } => {
                todo!()
            }
            DataKind::CharacterSet(set) => todo!(),
        }
    }
}

impl DataKind {}

impl From<DataKind> for ExpressionKind {
    fn from(e: DataKind) -> Self {
        Self::Data(Box::new(e))
    }
}

impl ExpressionKind {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::CharacterAny,
            "XID_START" => DataKind::Builtin(name.to_string()),
            _ => DataKind::Rule(RuleReference::new(name)),
        };
        ExpressionKind::Data(Box::new(data))
    }
    pub fn string(string: String) -> Self {
        let data = DataKind::String(string);
        ExpressionKind::Data(Box::new(data))
    }
    pub fn builtin(name: &str) -> Option<Self> {}
}

impl DataKind {
    pub fn set_tag(&mut self, tag: String) {
        match self {
            DataKind::CharacterAny => {}
            DataKind::String(_) => {}
            DataKind::Rule(r) => r.tag = tag,
            DataKind::Integer(_) => {}
            DataKind::Character(_) => {}
            DataKind::CharacterRange(_) => {}
            DataKind::CharacterSet(_) => {}
        }
    }
}
