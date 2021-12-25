use std::{
    fmt::{Debug, Formatter},
    ops::{AddAssign, BitAndAssign, Range},
};

use indexmap::set::IndexSet;

use character_set::CharacterSet;

use super::*;

pub mod choice;
pub mod concat;
pub mod debug;
pub mod expr;
pub mod unary;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    Unary(Box<UnaryExpression>),
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Data(Box<DataExpression>),
}

pub struct DataExpression {
    pub tag: String,
    pub kind: DataKind,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DataKind {
    AnyCharacter,
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
