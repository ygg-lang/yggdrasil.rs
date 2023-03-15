pub use self::{
    choice::ChoiceExpression,
    concat::ConcatExpression,
    unary::{UnaryExpression, YggdrasilOperator},
};
use crate::{
    data::{RuleReference, YggdrasilRegex, YggdrasilText},
    rule::{GrammarRule, GrammarRuleKind, YggdrasilIdentifier, YggdrasilMacroCall},
};
use num::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter},
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

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ExpressionKind {
    /// Any ignored rule
    Ignored,
    Choice(ChoiceExpression),
    Concat(ConcatExpression),
    Call(YggdrasilMacroCall),
    Unary(UnaryExpression),
    Rule(RuleReference),
    Text(YggdrasilText),
    /// Any character
    CharacterAny,
    CharacterRestOfLine,
    CharacterRange(RangeInclusive<char>),
    Integer(BigInt),
    Boolean(bool),
    Regex(YggdrasilRegex),
}

impl Debug for ExpressionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionKind::Ignored => f.write_str("IGNORED"),
            ExpressionKind::Choice(v) => Debug::fmt(v, f),
            ExpressionKind::Concat(v) => Debug::fmt(v, f),
            ExpressionKind::Call(v) => Debug::fmt(v, f),
            ExpressionKind::Unary(v) => Debug::fmt(v, f),
            ExpressionKind::Rule(v) => Debug::fmt(v, f),
            ExpressionKind::Text(v) => Debug::fmt(v, f),
            ExpressionKind::CharacterAny => f.write_str("ANY"),
            ExpressionKind::CharacterRestOfLine => f.write_str("ROL"),
            ExpressionKind::CharacterRange(v) => Debug::fmt(v, f),
            ExpressionKind::Integer(v) => Debug::fmt(v, f),
            ExpressionKind::Boolean(v) => Debug::fmt(v, f),
            ExpressionKind::Regex(v) => Debug::fmt(v, f),
        }
    }
}

impl From<ExpressionKind> for YggdrasilExpression {
    fn from(value: ExpressionKind) -> Self {
        Self { kind: value, remark: false, tag: None }
    }
}

impl YggdrasilExpression {
    pub fn integer<T>(int: T) -> Self
    where
        T: Into<BigInt>,
    {
        ExpressionKind::Integer(int.into()).into()
    }
    pub fn unary(mut base: YggdrasilExpression, o: YggdrasilOperator) -> Self {
        match base.kind {
            ExpressionKind::Unary(ref mut v) if base.tag.is_none() => {
                v.operators.push(o);
                base
            }
            _ => ExpressionKind::Unary(UnaryExpression { base: Box::new(base), operators: vec![o] }).into(),
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
