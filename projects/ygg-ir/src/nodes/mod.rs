use crate::{
    data::DataKind,
    nodes::{choice::ChoiceExpression, concat::ConcatExpression, unary::UnaryExpression},
    rule::{GrammarRule, GrammarRuleKind},
    FunctionExpression,
};
use convert_case::{Case, Casing};
use diagnostic_quick::{QError, QResult};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    mem::take,
    ops::{Add, BitAnd, BitOr, BitXor},
    slice::{Iter, IterMut},
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
    pub tag: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExpressionKind {
    Function(Box<FunctionExpression>),
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Unary(Box<UnaryExpression>),
    Rule(Box<RuleReference>),
    Data(Box<DataKind>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
