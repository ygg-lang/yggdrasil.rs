use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChoiceExpression {
    pub branches: IndexSet<ExpressionNode>,
}

impl Default for ChoiceExpression {
    fn default() -> Self {
        Self { branches: Default::default() }
    }
}

impl From<ChoiceExpression> for ExpressionNode {
    fn from(value: ChoiceExpression) -> Self {
        Self { kind: ExpressionKind::Choice(Box::new(value)), tag: "".to_string() }
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
    pub fn push(&mut self, other: ExpressionNode) {
        match other.kind {
            ExpressionKind::Choice(rhs) => self.branches.extend(rhs.branches.into_iter()),
            _ => {
                self.branches.insert(other);
            }
        }
    }
    pub fn to_node<S>(self, tag: S) -> ExpressionNode
    where
        S: Into<String>,
    {
        ExpressionNode { tag: tag.into(), kind: ExpressionKind::Choice(Box::new(self)) }
    }
}

impl GrammarRule {
    pub fn get_branches(&self) -> IndexMap<&str, &ExpressionNode> {
        let mut out = IndexMap::default();
        if self.kind == GrammarRuleKind::Union {
            match &self.body.kind {
                ExpressionKind::Choice(v) => {
                    for branch in &v.branches {
                        out.insert(branch.tag.as_str(), branch);
                    }
                }
                _ => {}
            }
        }
        out
    }
}

impl BitOr<Self> for ExpressionNode {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        bitor_wrapper(self, other)
    }
}

fn bitor_wrapper(mut lhs: ExpressionNode, mut rhs: ExpressionNode) -> ExpressionNode {
    match (&mut lhs.kind, &mut rhs.kind) {
        (ExpressionKind::Choice(a), _) => {
            a.push(rhs);
            lhs
        }
        (_, ExpressionKind::Choice(b)) => {
            b.push(lhs);
            rhs
        }
        (_, _) => {
            let choice = ChoiceExpression::new(lhs, rhs);
            ExpressionNode { kind: ExpressionKind::Choice(Box::new(choice)), tag: "".to_string() }
        }
    }
}
