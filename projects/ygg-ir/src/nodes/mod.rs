pub use self::{choice::ChoiceExpression, concat::ConcatExpression, unary::UnaryExpression};
use crate::{
    data::{DataKind, RegularExpression, RuleReference, YggdrasilText},
    rule::{GrammarRule, GrammarRuleKind, YggdrasilIdentifier},
    FunctionExpression,
};
use diagnostic_quick::{QError, QResult};
use indexmap::{IndexMap, IndexSet};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    mem::take,
    ops::{Add, BitAnd, BitOr, BitXor},
    slice::{Iter, IterMut},
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
    pub tag: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExpressionKind {
    Ignored,
    Function(Box<FunctionExpression>),
    Choice(Box<ChoiceExpression>),
    Concat(ConcatExpression),
    Unary(Box<UnaryExpression>),
    Rule(RuleReference),
    Text(YggdrasilText),
    Regex(RegularExpression),
    Data(Box<DataKind>),
}

impl From<YggdrasilText> for ExpressionNode {
    fn from(value: YggdrasilText) -> Self {
        Self { kind: ExpressionKind::Text(value), tag: "".to_string() }
    }
}

impl From<ConcatExpression> for ExpressionNode {
    fn from(value: ConcatExpression) -> Self {
        Self { kind: ExpressionKind::Concat(value), tag: "".to_string() }
    }
}

impl From<RegularExpression> for ExpressionNode {
    fn from(value: RegularExpression) -> Self {
        Self { kind: ExpressionKind::Regex(value), tag: "".to_string() }
    }
}

impl ExpressionNode {
    pub fn unary(mut base: ExpressionNode, o: Operator) -> Self {
        match base.kind {
            ExpressionKind::Unary(ref mut v) if base.tag.is_empty() => {
                v.operators.push(o);
                base
            }
            _ => Self { kind: ExpressionKind::Unary(Box::new(UnaryExpression { base, operators: vec![o] })), tag: "".to_string() },
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operator {
    /// ```ygg
    /// !e
    /// ```
    Negative,
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeat1,
    /// no such literal
    Boxing,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// ^e
    Remark,
    /// *e
    Recursive,
}
