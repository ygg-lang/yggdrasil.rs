use std::{
    fmt::{Debug, Formatter},
    ops::{AddAssign, BitAndAssign, Range},
};

use indexmap::set::IndexSet;

use character_set::CharacterSet;

use crate::frontend::{
    rule::node::{choice::ChoiceExpression, concat::ConcatExpression, unary::UnaryExpression},
    Rule,
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Expression {
    pub node_tag: Option<String>,
    pub node: Term,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Term {
    Unary(Box<UnaryExpression>),
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Data(Box<DataExpression>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DataExpression {
    String(String),
    Regex(String),
    Integer(isize),
    Character(char),
    CharacterRange(Range<char>),
    CharacterSet(CharacterSet),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Operator {
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeats1,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// ^e
    Mark,
    /// *e
    Recursive,
}
