use super::*;
use std::ops::{AddAssign, BitAndAssign};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        ExpressionBody::Concat(value).into()
    }
}

impl ConcatExpression {
    pub fn new(terms: Vec<YggdrasilExpression>) -> Option<Self> {
        if terms.is_empty() {
            return None;
        }
        return Some(Self { sequence: terms });
    }
    pub fn pair(lhs: impl Into<YggdrasilExpression>, rhs: impl Into<YggdrasilExpression>, soft: bool) -> Self {
        if soft {
            Self { sequence: vec![lhs.into(), YggdrasilExpression::ignored(), rhs.into()] }
        }
        else {
            Self { sequence: vec![lhs.into(), rhs.into()] }
        }
    }
    pub fn split(&self) -> (YggdrasilExpression, &[YggdrasilExpression]) {
        match self.sequence.split_first() {
            Some((head, rest)) => (head.clone(), rest),
            None => unreachable!("Empty nodes are illegal, make sure you use `ConcatExpression::new`"),
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
        match &mut self.body {
            ExpressionBody::Concat(this) if self.tag.is_none() && rhs.tag.is_none() => {
                this.sequence.push(YggdrasilExpression::ignored());
                match rhs.body {
                    ExpressionBody::Concat(that) => this.sequence.extend(that.sequence),
                    _ => this.sequence.push(rhs),
                }
                return;
            }
            _ => {}
        }
        *self = ConcatExpression { sequence: vec![self.clone(), YggdrasilExpression::ignored(), rhs] }.into()
    }
}

impl BitAnd for YggdrasilExpression {
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
        match &mut self.body {
            ExpressionBody::Concat(this) if self.tag.is_none() && rhs.tag.is_none() => {
                match rhs.body {
                    ExpressionBody::Concat(that) => this.sequence.extend(that.sequence),
                    _ => this.sequence.push(rhs),
                }
                return;
            }
            _ => {}
        }
        *self = ConcatExpression { sequence: vec![self.clone(), rhs] }.into()
    }
}
