use crate::nodes::ExpressionNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FunctionExpression {
    pub name: String,
    pub args: ExpressionNode,
}
