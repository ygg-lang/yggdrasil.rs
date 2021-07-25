use crate::codegen::rule::Rule;
use std::{
    fmt::{Debug, Formatter},
    ops::AddAssign,
};
use std::ops::BitAndAssign;
use yggdrasil_bootstrap::ast::*;

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
    pub ty: Option<Symbol>,
    pub field: Option<Symbol>,
    pub node: RefinedExpression,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExpressionTag {
    pub tag: Symbol,
    pub mode: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum RefinedExpression {
    Data(Box<RefinedData>),
    Unary(Box<RefinedUnary>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>),
}

#[derive(Clone, Eq, PartialEq)]
pub struct RefinedChoice {
    pub inner: Set<ExpressionNode>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedConcat {
    pub base: ExpressionNode,
    pub rest: Vec<(bool, ExpressionNode)>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedUnary {
    pub base: ExpressionNode,
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
    /// ^e
    Mark,
    /// *e
    Recursive,
}

#[derive(Clone)]
pub enum RefinedData {
    Symbol(SymbolPath),
    String(String),
    Regex(String),
    Integer(isize),
}
