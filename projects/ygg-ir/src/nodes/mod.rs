pub use self::{
    choice::ChoiceExpression,
    concat::ConcatExpression,
    unary::{UnaryExpression, YggdrasilOperator},
};
use crate::{
    data::{RuleReference, YggdrasilRegex, YggdrasilText},
    rule::{GrammarRule, YggdrasilIdentifier, YggdrasilMacroCall},
};
use convert_case::{Case, Casing};
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
    /// If it's a `tag:Rule`
    pub tag: Option<YggdrasilIdentifier>,
    /// If it's a `^rule`
    pub remark: bool,
    /// Main body of the expression
    pub body: ExpressionBody,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ExpressionBody {
    Choice(ChoiceExpression),
    Concat(ConcatExpression),
    Call(YggdrasilMacroCall),
    Unary(UnaryExpression),
    Rule(RuleReference),
    Text(YggdrasilText),
    /// Any ignored rule
    Ignored,
    /// Any character
    CharacterAny,
    CharacterRestOfLine,
    CharacterRange(RangeInclusive<char>),
    Integer(BigInt),
    Boolean(bool),
    Regex(YggdrasilRegex),
}

impl Debug for ExpressionBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionBody::Ignored => f.write_str("IGNORED"),
            ExpressionBody::Choice(v) => Debug::fmt(v, f),
            ExpressionBody::Concat(v) => Debug::fmt(v, f),
            ExpressionBody::Call(v) => Debug::fmt(v, f),
            ExpressionBody::Unary(v) => Debug::fmt(v, f),
            ExpressionBody::Rule(v) => Debug::fmt(v, f),
            ExpressionBody::Text(v) => Debug::fmt(v, f),
            ExpressionBody::CharacterAny => f.write_str("ANY"),
            ExpressionBody::CharacterRestOfLine => f.write_str("ROL"),
            ExpressionBody::CharacterRange(v) => Debug::fmt(v, f),
            ExpressionBody::Integer(v) => Debug::fmt(v, f),
            ExpressionBody::Boolean(v) => Debug::fmt(v, f),
            ExpressionBody::Regex(v) => Debug::fmt(v, f),
        }
    }
}

impl From<ExpressionBody> for YggdrasilExpression {
    fn from(value: ExpressionBody) -> Self {
        Self { body: value, remark: false, tag: None }
    }
}

impl YggdrasilExpression {
    pub fn integer<T>(int: T) -> Self
    where
        T: Into<BigInt>,
    {
        ExpressionBody::Integer(int.into()).into()
    }
    pub fn unary(mut base: YggdrasilExpression, o: YggdrasilOperator) -> Self {
        match base.body {
            ExpressionBody::Unary(ref mut v) if base.tag.is_none() => {
                v.operators.push(o);
                base
            }
            _ => ExpressionBody::Unary(UnaryExpression { base: Box::new(base), operators: vec![o] }).into(),
        }
    }
    pub fn ignored() -> Self {
        ExpressionBody::Ignored.into()
    }
    pub fn any() -> Self {
        ExpressionBody::CharacterAny.into()
    }
    pub fn rol() -> Self {
        ExpressionBody::CharacterRestOfLine.into()
    }
    pub fn boolean(bool: bool) -> Self {
        ExpressionBody::Boolean(bool).into()
    }
    /// Get the name if it is a union variant.
    pub fn variant_name(&self, rule: &GrammarRule, index: usize) -> String {
        match &self.tag {
            Some(s) => return s.text.to_case(Case::Pascal),
            None => match &self.body {
                ExpressionBody::Rule(v) => return v.name.text.to_case(Case::Pascal),
                _ => {}
            },
        }
        format!("{}{}", rule.name.text, index).to_case(Case::Pascal)
    }
}
