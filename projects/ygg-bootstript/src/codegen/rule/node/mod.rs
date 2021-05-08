use crate::ast::*;
use std::ops::AddAssign;

mod choice;
mod concat;
mod expr;
mod unary;

#[derive(Clone, Debug)]
pub struct ExpressionNode {
    pub tag: Option<ExpressionTag>,
    pub ty: Option<Identifier>,
    pub field: Option<Identifier>,
    pub node: RefinedExpression,
}
#[derive(Clone, Debug)]
pub struct ExpressionTag {
    tag: Identifier,
    mode: String,
}

#[derive(Clone, Debug)]
pub enum RefinedExpression {
    Data(Box<RefinedData>),
    Unary(Box<RefinedUnary>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>),
}

#[derive(Clone, Debug)]
pub struct RefinedChoice {
    pub inner: Vec<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct RefinedConcat {
    pub inner: Vec<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct RefinedUnary {
    base: ExpressionNode,
    prefix: Vec<String>,
    suffix: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum RefinedData {
    Identifier(Identifier),
    String(String),
    Regex(String),
    Integer(usize),
}
