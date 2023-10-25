use crate::rule::YggdrasilCounter;
use convert_case::{Case, Casing};
use num::BigInt;
use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    ops::{Add, BitAnd, BitOr, RangeInclusive},
};

use crate::{
    data::{RuleReference, YggdrasilRegex, YggdrasilText},
    rule::{GrammarRule, YggdrasilIdentifier, YggdrasilMacroCall},
};

pub use self::{
    choice::ChoiceExpression,
    concat::ConcatExpression,
    unary::{UnaryExpression, YggdrasilOperator},
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilExpression {
    /// If it's a `tag:Rule`
    pub tag: Option<YggdrasilIdentifier>,
    /// If it's a `^rule`
    pub remark: bool,
    /// Main body of the expression
    pub body: ExpressionBody,
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StreamControl {
    StartOfInput,
    EndOfInput,
    RestOfLine,
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionBody {
    Choice(ChoiceExpression),
    Concat(ConcatExpression),
    Call(YggdrasilMacroCall),
    Unary(UnaryExpression),
    Rule(RuleReference),
    Text(YggdrasilText),
    /// Any ignored rule
    Hidden,
    /// Any character
    CharacterAny,
    CharacterRange(RangeInclusive<char>),
    Stream(StreamControl),
    Integer(BigInt),
    Boolean(bool),
    Regex(YggdrasilRegex),
}

impl Debug for YggdrasilExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Expression");
        match &self.tag {
            Some(s) => f.field("tag", s),
            None => f.field("tag", &Option::<()>::None),
        };
        f.field("remark", &self.remark);
        f.field("body", &self.body);
        f.finish()
    }
}

impl Debug for ExpressionBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionBody::Hidden => f.write_str("HIDE"),
            ExpressionBody::Choice(v) => Debug::fmt(v, f),
            ExpressionBody::Concat(v) => Debug::fmt(v, f),
            ExpressionBody::Call(v) => Debug::fmt(v, f),
            ExpressionBody::Unary(v) => Debug::fmt(v, f),
            ExpressionBody::Rule(v) => Debug::fmt(v, f),
            ExpressionBody::Text(v) => Debug::fmt(v, f),
            ExpressionBody::CharacterAny => f.write_str("ANY"),
            ExpressionBody::CharacterRange(v) => Debug::fmt(v, f),
            ExpressionBody::Integer(v) => Debug::fmt(v, f),
            ExpressionBody::Boolean(v) => Debug::fmt(v, f),
            ExpressionBody::Regex(v) => Debug::fmt(v, f),
            ExpressionBody::Stream(v) => match v {
                StreamControl::StartOfInput => f.write_str("ANY"),
                StreamControl::EndOfInput => f.write_str("ANY"),
                StreamControl::RestOfLine => f.write_str("ROL"),
            },
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
    pub fn ignored() -> Self {
        ExpressionBody::Hidden.into()
    }
    pub fn any() -> Self {
        ExpressionBody::CharacterAny.into()
    }
    pub fn start_of_input() -> Self {
        ExpressionBody::Stream(StreamControl::StartOfInput).into()
    }
    pub fn end_of_line() -> Self {
        ExpressionBody::Stream(StreamControl::StartOfInput).into()
    }
    pub fn rest_of_line() -> Self {
        ExpressionBody::Stream(StreamControl::RestOfLine).into()
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
