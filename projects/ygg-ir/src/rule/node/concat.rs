use std::{
    mem::swap,
    ops::{Add, BitAnd},
};

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
    pub fn push(&mut self, rhs: ExpressionNode, soft: bool) {
        if soft {
            self.sequence.push(ExpressionNode::ignored())
        }
        self.sequence.push(rhs);
    }
    pub fn append(&mut self, lhs: ExpressionNode, soft: bool) {
        let mut v = vec![lhs];
        swap(&mut v, &mut self.sequence);
        if soft {
            self.sequence.push(ExpressionNode::ignored())
        }
        self.sequence.extend(v.into_iter())
    }
}

impl Add<Self> for ExpressionNode {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self::Output {
        join(self, other, true)
    }
}

impl BitAnd<Self> for ExpressionNode {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let mut lhs = self;
        let mut rhs = other;
        join(lhs, rhs, false)
    }
}

fn join(mut lhs: ExpressionNode, mut rhs: ExpressionNode, soft: bool) -> ExpressionNode {
    match (&mut lhs.kind, &mut rhs.kind) {
        (ExpressionKind::Concat(a), _) => {
            a.extend(rhs.kind, soft);
            lhs
        }
        (_, ExpressionKind::Concat(b)) => {
            a.push(rhs, soft);
            lhs
        }
    }
}
