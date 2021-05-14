use crate::ast::*;
use std::ops::AddAssign;
use std::fmt::{Debug, Formatter};
use crate::codegen::rule::YGGRule;

mod choice;
mod concat;
mod expr;
mod debug;
mod unary;

#[derive(Clone)]
pub struct ExpressionNode {
    pub inline_token: bool,
    pub tag: Option<ExpressionTag>,
    pub ty: Option<Identifier>,
    pub field: Option<Identifier>,
    pub node: RefinedExpression,
}

#[derive(Clone, Debug)]
pub struct ExpressionTag {
   pub tag: Identifier,
    pub mode: String,
}

#[derive(Clone)]
pub enum RefinedExpression {
    Data(Box<RefinedData>),
    Unary(Box<RefinedUnary>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>),
}

#[derive(Clone)]
pub struct RefinedChoice {
    pub inner: Vec<ExpressionNode>,
}

#[derive(Clone)]
pub struct RefinedConcat {
    pub inner: Vec<ExpressionNode>,
}

#[derive(Clone)]
pub struct RefinedUnary {
  pub  base: ExpressionNode,
    pub  ops: Vec<Operator>,
}

#[derive(Copy, Clone)]
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
    Recursive
}

#[derive(Clone)]
pub enum RefinedData {
    Identifier(Identifier),
    String(String),
    Regex(String),
    Integer(usize),
}
