use super::*;

use std::ops::BitOrAssign;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChoiceExpression {
    pub branches: Vec<YggdrasilExpression>,
}

impl Default for ChoiceExpression {
    fn default() -> Self {
        Self { branches: Default::default() }
    }
}

impl From<ChoiceExpression> for YggdrasilExpression {
    fn from(value: ChoiceExpression) -> Self {
        ExpressionBody::Choice(value).into()
    }
}

impl Hash for ChoiceExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.branches.iter().for_each(|e| e.hash(state))
    }
}

impl ChoiceExpression {
    pub fn new(terms: Vec<YggdrasilExpression>) -> Option<Self> {
        if terms.is_empty() {
            return None;
        }
        return Some(Self { branches: terms });
    }
    pub fn pair(lhs: impl Into<YggdrasilExpression>, rhs: impl Into<YggdrasilExpression>) -> Self {
        Self { branches: vec![lhs.into(), rhs.into()] }
    }
    pub fn split(&self) -> (YggdrasilExpression, &[YggdrasilExpression]) {
        match self.branches.split_first() {
            Some((head, rest)) => (head.clone(), rest),
            None => unreachable!("Empty nodes are illegal, make sure you use `ChoiceExpression::new`"),
        }
    }
}

impl BitOr for YggdrasilExpression {
    type Output = Self;
    /// `a | b`
    fn bitor(mut self, other: Self) -> Self::Output {
        self |= other;
        self
    }
}

impl BitOrAssign for YggdrasilExpression {
    fn bitor_assign(&mut self, rhs: Self) {
        match &mut self.body {
            ExpressionBody::Choice(this) => {
                match rhs.body {
                    ExpressionBody::Choice(that) => this.branches.extend(that.branches),
                    _ => this.branches.push(rhs),
                }
                return;
            }
            _ => {}
        }
        *self = ChoiceExpression { branches: vec![self.clone(), rhs] }.into()
    }
}
