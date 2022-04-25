use std::ops::{Add, BitAnd};

use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatExpression {
    pub sequence: Vec<ExpressionNode>,
}

impl ConcatExpression {
    pub fn new(lhs: impl Into<ExpressionNode>, rhs: impl Into<ExpressionNode>, soft: bool) -> Self {
        let mut sequence = vec![];
        sequence.push(lhs.into());
        if soft {
            sequence.push(ExpressionNode::ignored())
        }
        sequence.push(rhs.into());
        Self { sequence }
    }
}

impl Add<Self> for ExpressionNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let concat = ConcatExpression::new(self, rhs, true);
        ExpressionNode { kind: ExpressionKind::Concat(Box::new(concat)), tag: "".to_string() }
    }
}

impl BitAnd<Self> for ExpressionNode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let concat = ConcatExpression::new(self, rhs, false);
        ExpressionNode { kind: ExpressionKind::Concat(Box::new(concat)), tag: "".to_string() }
    }
}
