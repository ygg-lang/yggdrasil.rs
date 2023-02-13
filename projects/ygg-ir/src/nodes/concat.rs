use super::*;
use std::ops::{AddAssign, BitAndAssign};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ConcatExpression {
    sequence: Vec<ExpressionNode>,
}

impl<T> FromIterator<T> for ConcatExpression
where
    T: Into<ExpressionNode>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self { sequence: iter.into_iter().map(|v| v.into()).collect() }
    }
}

impl<'i> IntoIterator for &'i ConcatExpression {
    type Item = &'i ExpressionNode;
    type IntoIter = Iter<'i, ExpressionNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.iter()
    }
}

impl<'i> IntoIterator for &'i mut ConcatExpression {
    type Item = &'i mut ExpressionNode;
    type IntoIter = IterMut<'i, ExpressionNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.iter_mut()
    }
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
    pub fn to_node<S>(self, tag: S) -> ExpressionNode
    where
        S: Into<String>,
    {
        ExpressionNode { tag: tag.into(), kind: ExpressionKind::Concat(self) }
    }
}

impl Add<Self> for ExpressionNode {
    type Output = Self;
    /// soft concat
    #[inline(never)]
    fn add(self, other: Self) -> Self::Output {
        join(self, other, true)
    }
}

/// a ~ b ~ c
impl AddAssign for ExpressionNode {
    /// - atomic:  a b = a b
    /// - combine: a b = a ignore b
    fn add_assign(&mut self, rhs: Self) {}
}

impl BitAnd<Self> for ExpressionNode {
    type Output = Self;
    fn bitand(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl BitAndAssign for ExpressionNode {
    /// p:a q:(b c) | d e
    fn bitand_assign(&mut self, rhs: Self) {
        match &mut self.kind {
            ExpressionKind::Concat(this) if self.tag.is_empty() && rhs.tag.is_empty() => {
                match rhs.kind {
                    ExpressionKind::Concat(that) => this.sequence.extend(that.sequence),
                    _ => this.sequence.push(rhs),
                }
                return;
            }
            _ => {}
        }
        *self = ConcatExpression { sequence: vec![self.clone(), rhs] }.into()
    }
}

// add extra ignore if is a soft concat
#[inline(always)]
fn join(mut lhs: ExpressionNode, mut rhs: ExpressionNode, soft: bool) -> ExpressionNode {
    match (&mut lhs.kind, &mut rhs.kind) {
        (ExpressionKind::Concat(a), ExpressionKind::Concat(b)) => {
            if soft {
                a.sequence.push(ExpressionNode::ignored())
            }
            a.sequence.extend(b.sequence.iter().cloned());
            lhs
        }
        (ExpressionKind::Concat(a), _) => {
            if soft {
                a.sequence.push(ExpressionNode::ignored())
            }
            a.sequence.push(rhs);
            lhs
        }
        // a:A b:B
        (_, ExpressionKind::Concat(b)) => {
            let mut sequence = vec![];
            sequence.push(lhs);
            if soft {
                sequence.push(ExpressionNode::ignored())
            }
            sequence.extend(b.sequence.iter().cloned());
            ConcatExpression { sequence }.to_node("")
        }
        (_, _) => {
            let mut sequence = vec![];
            sequence.push(lhs);
            if soft {
                sequence.push(ExpressionNode::ignored())
            }
            sequence.push(rhs);
            ConcatExpression { sequence }.to_node("")
        }
    }
}
