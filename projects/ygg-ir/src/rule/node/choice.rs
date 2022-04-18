use super::*;
use std::ops::BitOr;

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
                ExpressionNode { kind: ExpressionKind::Choice(a), branch_tag: "".to_string(), node_tag: "".to_string() }
            }
            (ExpressionKind::Choice(mut a), b) | (b, ExpressionKind::Choice(mut a)) => {
                a.push(ExpressionNode { kind: b, branch_tag: other.branch_tag, node_tag: other.node_tag });
                ExpressionNode { kind: ExpressionKind::Choice(a), branch_tag: self.branch_tag, node_tag: self.node_tag }
            }
            (a, b) => {
                let new = ChoiceExpression::new(
                    ExpressionNode { kind: a, branch_tag: self.branch_tag, node_tag: self.node_tag },
                    ExpressionNode { kind: b, branch_tag: other.branch_tag, node_tag: other.node_tag },
                );
                ExpressionNode { kind: ExpressionKind::Choice(Box::new(new)), branch_tag: "".to_string(), node_tag: "".to_string() }
            }
        }
    }
}

// impl Expression {
//     #[inline]
//     pub fn choice(lhs: Term, rhs: Term) -> Self {
//         let mut base = Expression::from(lhs).as_choice();
//         base.get_choice_mut().map(|e| e.add_assign(Expression::from(rhs)));
//         return base;
//     }
//     fn as_choice(self) -> Self {
//         if let Some(_) = self.get_choice() {
//             return self;
//         }
//         return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: Term::concat(self) };
//     }
// }
//
// impl Term {
//     pub fn choice(base: Expression) -> Self {
//         let mut inner = IndexSet::default();
//         inner.insert(base);
//         Self::Choice(Box::new(ChoiceExpression { inner }))
//     }
// }
//
// impl From<Term> for ChoiceExpression {
//     fn from(e: Term) -> Self {
//         Self::from(Expression::from(e))
//     }
// }
//
// impl From<Expression> for ChoiceExpression {
//     fn from(e: Expression) -> Self {
//         match e.get_choice() {
//             Some(s) => Self { inner: s.inner },
//             None => {
//                 let mut inner = IndexSet::new();
//                 inner.insert(e);
//                 Self { inner }
//             }
//         }
//     }
// }
//
// impl AddAssign<Expression> for ChoiceExpression {
//     fn add_assign(&mut self, rhs: Expression) {
//         match rhs.get_choice() {
//             Some(c) => c.inner.into_iter().for_each(|e| {
//                 self.inner.insert(e);
//             }),
//             None => {
//                 self.inner.insert(rhs);
//             }
//         };
//     }
// }
