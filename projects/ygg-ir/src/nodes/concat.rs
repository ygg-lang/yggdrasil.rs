use super::*;
use std::ops::{AddAssign, BitAndAssign};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ConcatExpression {
    pub sequence: Vec<YggdrasilExpression>,
}

impl<T> FromIterator<T> for ConcatExpression
where
    T: Into<YggdrasilExpression>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self { sequence: iter.into_iter().map(|v| v.into()).collect() }
    }
}

impl From<ConcatExpression> for YggdrasilExpression {
    fn from(value: ConcatExpression) -> Self {
        ExpressionKind::Concat(value).into()
    }
}

impl ConcatExpression {
    pub fn new(lhs: impl Into<YggdrasilExpression>, rhs: impl Into<YggdrasilExpression>, soft: bool) -> Self {
        let mut sequence = vec![];
        sequence.push(lhs.into());
        if soft {
            sequence.push(YggdrasilExpression::ignored())
        }
        sequence.push(rhs.into());
        Self { sequence }
    }
    pub fn split(&self) -> (YggdrasilExpression, Vec<YggdrasilExpression>) {
        match self.sequence.split_first() {
            Some((head, rest)) => (head.clone(), rest.to_vec()),
            None => unreachable!("invalid empty"),
        }
    }
}

impl Add for YggdrasilExpression {
    type Output = Self;
    /// `a ~ b`
    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for YggdrasilExpression {
    /// `a ~ b ~ c`
    fn add_assign(&mut self, rhs: Self) {
        match &mut self.kind {
            ExpressionKind::Concat(this) if self.tag.is_none() && rhs.tag.is_none() => {
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

impl BitAnd<Self> for YggdrasilExpression {
    type Output = Self;
    /// `a b`
    fn bitand(mut self, other: Self) -> Self::Output {
        self &= other;
        self
    }
}

impl BitAndAssign for YggdrasilExpression {
    /// `p:a q:(b c) | d e`
    fn bitand_assign(&mut self, rhs: Self) {
        match &mut self.kind {
            ExpressionKind::Concat(this) if self.tag.is_none() && rhs.tag.is_none() => {
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
