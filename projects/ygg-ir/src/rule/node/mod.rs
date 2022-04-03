use super::*;

pub mod choice;
pub mod concat;
pub mod debug;
pub mod expr;
pub mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
    pub branch_tag: String,
    pub node_tag: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExpressionKind {
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Unary(Box<UnaryExpression>),
    Rule(Box<RuleReference>),
    Data(Box<DataKind>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    /// ```ygg
    /// !e
    /// ```
    Negative,
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeat1,
    /// no such literal
    Boxing,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// ^e
    Remark,
    /// *e
    Recursive,
}
