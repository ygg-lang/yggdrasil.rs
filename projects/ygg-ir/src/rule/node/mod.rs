use super::*;

pub mod choice;
pub mod concat;
pub mod data;
pub mod debug;
pub mod expr;
pub mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExpressionKind {
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Unary(Box<UnaryExpression>),
    Data(Box<DataKind>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operator {
    /// ```
    /// !e
    /// ```
    Negative,
    /// e?
    Optional,
    /// e*
    Repeat,
    /// e+
    Repeat1,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// ^e
    Remark,
    /// *e
    Recursive,
}
