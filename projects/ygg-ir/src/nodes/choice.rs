use super::*;

use std::ops::BitOrAssign;

#[derive(Debug, Clone, Eq, PartialEq)]
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
        ExpressionKind::Choice(value).into()
    }
}

impl Hash for ChoiceExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.branches.iter().for_each(|e| e.hash(state))
    }
}

impl ChoiceExpression {
    pub fn new(lhs: impl Into<YggdrasilExpression>, rhs: impl Into<YggdrasilExpression>) -> Self {
        Self { branches: vec![lhs.into(), rhs.into()] }
    }
    pub fn split(&self) -> (&YggdrasilExpression, &[YggdrasilExpression]) {
        self.branches.split_first().expect("invalid branches")
    }
}

impl GrammarRule {
    pub fn get_branches(&self) -> IndexMap<&str, &YggdrasilExpression> {
        let mut out = IndexMap::default();
        if self.kind == GrammarRuleKind::Union {
            // match &self.body.kind {
            //     ExpressionKind::Choice(v) => {
            //         for branch in &v.branches {
            //             out.insert(branch.tag.as_str(), branch);
            //         }
            //     }
            //     _ => {}
            // }
        }
        out
    }
}

impl BitOr<Self> for YggdrasilExpression {
    type Output = Self;
    /// `a | b`
    fn bitor(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl BitOrAssign for YggdrasilExpression {
    fn bitor_assign(&mut self, mut rhs: Self) {
        match &mut self.kind {
            ExpressionKind::Choice(this) if self.tag.is_none() && rhs.tag.is_none() => {
                match rhs.kind {
                    ExpressionKind::Concat(that) => this.branches.extend(that.sequence),
                    _ => this.branches.push(rhs),
                }
                return;
            }
            _ => {}
        }
        *self = ChoiceExpression { branches: vec![self.clone(), rhs] }.into()
    }
}
