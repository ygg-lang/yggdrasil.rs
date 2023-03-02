pub use self::{
    choice::ChoiceExpression,
    concat::ConcatExpression,
    unary::{UnaryExpression, YggdrasilOperator},
};
use crate::{
    data::{RuleReference, YggdrasilRegex, YggdrasilText},
    rule::{GrammarRule, GrammarRuleKind, YggdrasilIdentifier},
    FunctionExpression,
};
use num::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::{Add, BitAnd, BitOr, RangeInclusive},
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct YggdrasilExpression {
    pub kind: ExpressionKind,
    /// If it's a `^rule`
    pub remark: bool,
    /// If it's a `tag:Rule`
    pub tag: Option<YggdrasilIdentifier>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExpressionKind {
    /// Any ignored rule
    Ignored,
    Function(Box<FunctionExpression>),
    Choice(ChoiceExpression),
    Concat(ConcatExpression),
    Unary(Box<UnaryExpression>),
    Rule(RuleReference),
    Text(YggdrasilText),
    /// Any character
    CharacterAny,
    CharacterRange(RangeInclusive<char>),
    Integer(BigInt),
    Boolean(bool),
    Regex(YggdrasilRegex),
}

impl From<ExpressionKind> for YggdrasilExpression {
    fn from(value: ExpressionKind) -> Self {
        Self { kind: value, remark: false, tag: None }
    }
}

impl YggdrasilExpression {
    pub fn unary(mut base: YggdrasilExpression, o: YggdrasilOperator) -> Self {
        match base.kind {
            ExpressionKind::Unary(ref mut v) if base.tag.is_none() => {
                v.operators.push(o);
                base
            }
            _ => ExpressionKind::Unary(Box::new(UnaryExpression { base, operators: vec![o] })).into(),
        }
    }
    pub fn ignored() -> Self {
        ExpressionKind::Ignored.into()
    }
    pub fn any() -> Self {
        ExpressionKind::CharacterAny.into()
    }
    pub fn boolean(bool: bool) -> Self {
        ExpressionKind::Boolean(bool).into()
    }
}
