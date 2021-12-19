use crate::frontend::{Rule};
use std::{
    fmt::{Debug, Formatter},
    ops::{AddAssign, BitAndAssign},
};
use yggdrasil_bootstrap::parser::*;
use indexmap::set::IndexSet;

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TermNode {
    pub node_tag: Option<String>,
    pub node: ExpressionNode,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ExpressionNode {
    Unary(Box<RefinedUnary>),
    Choice(Box<ChoiceNode>),
    Concat(Box<RefinedConcat>),
    Data(Box<DataNode>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DataNode {
    String(String),
    Regex(String),
    Integer(isize),
    CharacterSet(Charset)
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Charset {
    pub
}



#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedConcat {
    pub base: TermNode,
    pub rest: Vec<(bool, TermNode)>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedUnary {
    pub base: TermNode,
    pub ops: Vec<Operator>,
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

