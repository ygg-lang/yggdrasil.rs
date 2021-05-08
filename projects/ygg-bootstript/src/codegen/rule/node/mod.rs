use crate::ast::*;
use std::ops::AddAssign;

mod expr;
mod choice;
mod concat;
mod unary;

#[derive(Clone, Debug)]
pub struct ExpressionNode {
    tag: Option<ExpressionTag>,
    ty: Option<ExpressionType>,
    field: Option<Identifier>,
    node :RefinedExpression
}
#[derive(Clone, Debug)]
pub struct ExpressionTag {
    tag: String,
    mode: String,
}
#[derive(Clone, Debug)]
pub struct ExpressionType {
    ty: String
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
    inner: Vec<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct RefinedConcat {
    inner: Vec<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct RefinedUnary {
    base: ExpressionNode,
    prefix: Vec<String>,
    suffix: Vec<String>
}

#[derive(Clone, Debug)]
pub enum RefinedData {
    Identifier(Identifier),
    String(String),
    Regex(String),
    Integer(usize),
}


