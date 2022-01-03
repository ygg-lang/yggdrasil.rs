use std::{
    fmt::{Debug, Formatter},
    ops::{AddAssign, BitAndAssign, Range},
};

use character_set::CharacterSet;
use indexmap::set::IndexSet;
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

use super::*;

pub mod choice;
pub mod concat;
pub mod data;
pub mod debug;
pub mod expr;
pub mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Expression {
    Unary(Box<UnaryExpression>),
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Data(Box<DataExpression>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
