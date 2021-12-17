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
pub struct ASTNode {
    pub inline_token: bool,
    pub node_tag: Option<String>,
    pub node: ASTExpression,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExpressionTag {
    pub name: Symbol,
    pub mode: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ASTExpression {
    Data(Box<RefinedData>),
    Unary(Box<RefinedUnary>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>),
}

#[derive(Clone, Eq, PartialEq)]
pub struct RefinedChoice {
    pub inner: IndexSet<ASTNode>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedConcat {
    pub base: ASTNode,
    pub rest: Vec<(bool, ASTNode)>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedUnary {
    pub base: ASTNode,
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

#[derive(Clone)]
pub enum RefinedData {
    Symbol(SymbolPath),
    String(String),
    Regex(String),
    Integer(isize),
}
