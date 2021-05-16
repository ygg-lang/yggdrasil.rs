use crate::{ast::*, codegen::rule::YGGRule};
use std::{
    fmt::{Debug, Formatter},
    ops::AddAssign,
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

// pub type Set<V> = std::collections::HashSet<V>;
pub type Set<V> = indexmap::IndexSet<V>;


#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ExpressionNode {
    pub inline_token: bool,
    pub tag: Option<ExpressionTag>,
    pub ty: Option<Identifier>,
    pub field: Option<Identifier>,
    pub node: RefinedExpression,
}

#[derive(Clone, Debug,Eq, PartialEq, Hash)]
pub struct ExpressionTag {
    pub tag: Identifier,
    pub mode: String,
}

#[derive(Clone,Eq, PartialEq, Hash)]
pub enum RefinedExpression {
    Data(Box<RefinedData>),
    Unary(Box<RefinedUnary>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>),
}

#[derive(Clone,Eq, PartialEq)]
pub struct RefinedChoice {
    pub inner: Set<ExpressionNode>,
}

#[derive(Clone,Eq, PartialEq, Hash)]
pub struct RefinedConcat {
    pub inner: Vec<ExpressionNode>,
}

#[derive(Clone,Eq, PartialEq, Hash)]
pub struct RefinedUnary {
    pub base: ExpressionNode,
    pub ops: Vec<Operator>,
}

#[derive(Copy, Clone,Eq, PartialEq, Hash)]
pub enum Operator {
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeats1,
    /// ^e
    Mark,
    /// *e
    Recursive,
}

#[derive(Clone)]
pub enum RefinedData {
    Identifier(Identifier),
    String(String),
    Regex(String),
    Integer(usize),
}
