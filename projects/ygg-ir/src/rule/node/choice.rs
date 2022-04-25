use std::ops::BitOr;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChoiceExpression {
    pub branches: IndexSet<ExpressionNode>,
}

impl Default for ChoiceExpression {
    fn default() -> Self {
        Self { branches: Default::default() }
    }
}

impl Hash for ChoiceExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.branches.iter().for_each(|e| e.hash(state))
    }
}

impl ChoiceExpression {
    pub fn new(lhs: impl Into<ExpressionNode>, rhs: impl Into<ExpressionNode>) -> Self {
        let mut branches = IndexSet::default();
        branches.insert(lhs.into());
        branches.insert(rhs.into());
        Self { branches }
    }

    pub fn push(&mut self, e: impl Into<ExpressionNode>) {
        self.branches.insert(e.into());
    }
}

impl BitOr<Self> for ExpressionNode {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        match (self.kind, other.kind) {
            (ExpressionKind::Choice(mut a), ExpressionKind::Choice(b)) => {
                a.branches.extend(b.branches.into_iter());
                ExpressionNode { kind: ExpressionKind::Choice(a), tag: "".to_string() }
            }
            (ExpressionKind::Choice(mut a), b) | (b, ExpressionKind::Choice(mut a)) => {
                a.push(ExpressionNode { kind: b, tag: other.tag });
                ExpressionNode { kind: ExpressionKind::Choice(a), tag: self.tag }
            }
            (a, b) => {
                let new = ChoiceExpression::new(ExpressionNode { kind: a, tag: self.tag }, ExpressionNode { kind: b, tag: other.tag });
                ExpressionNode { kind: ExpressionKind::Choice(Box::new(new)), tag: "".to_string() }
            }
        }
    }
}
