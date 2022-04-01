use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    ops::Range,
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
    CharacterRange(Range<char>),
    CharacterSet(CharacterSet),
}

impl Display for DataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Integer(i) => write!(f, "{}", i),
            DataKind::String(s) => string_display(s, f),
            DataKind::Rule(rule) => Display::fmt(rule, f),
            DataKind::CharacterAny => write!(f, "ANY"),
            DataKind::Character(c) => write!(f, "{:?}", c),
            DataKind::CharacterRange(range) => char_range_display(range, f),
            DataKind::CharacterBuiltin(set) => write!(f, "{}", set),
            DataKind::CharacterSet(set) => char_set_display(set, f),
        }
    }
}

impl DataKind {}

impl From<DataKind> for ExpressionNode {
    fn from(e: DataKind) -> Self {
        Self::Data(Box::new(e))
    }
}

impl ExpressionNode {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::CharacterAny,
            "XID_START" => {
                todo!();
                // DataKind::CharacterSet(name.to_string())
            }
            _ => DataKind::Rule(RuleReference::new(name)),
        };
        ExpressionNode::Data(Box::new(data))
    }
    pub fn string(string: String) -> Self {
        let data = DataKind::String(string);
        ExpressionNode::Data(Box::new(data))
    }
    pub fn builtin(name: &str) -> Option<Self> {
        let builtin = &["XID_START"];
        if builtin.contains(&name) {
            let data = DataKind::CharacterBuiltin(name.to_string());
            Some(ExpressionNode::Data(Box::new(data)))
        }
        else {
            return None;
        }
    }
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
            DataKind::CharacterBuiltin(_) => {}
        }
    }
}
