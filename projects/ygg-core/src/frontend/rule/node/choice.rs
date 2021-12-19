use super::*;


#[derive(Clone, Eq, PartialEq)]
pub struct ChoiceNode {
    pub inner: IndexSet<TermNode>,
}

impl TermNode {
    #[inline]
    pub fn choice(lhs: ExpressionNode, rhs: ExpressionNode) -> Self {
        let mut base = TermNode::from(lhs).as_choice();
        base.get_choice_mut().map(|e| e.add_assign(TermNode::from(rhs)));
        return base;
    }
    fn as_choice(self) -> Self {
        if let Some(_) = self.get_choice() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: ExpressionNode::concat(self) };
    }
}

impl ExpressionNode {
    pub fn choice(base: TermNode) -> Self {
        let mut inner = IndexSet::default();
        inner.insert(base);
        Self::Choice(Box::new(ChoiceNode { inner }))
    }
}

impl From<ExpressionNode> for ChoiceNode {
    fn from(e: ExpressionNode) -> Self {
        Self::from(TermNode::from(e))
    }
}

impl From<TermNode> for ChoiceNode {
    fn from(e: TermNode) -> Self {
        match e.get_choice() {
            Some(s) => Self { inner: s.inner },
            None => {
                let mut inner = IndexSet::new();
                inner.insert(e);
                Self { inner }
            }
        }
    }
}

impl AddAssign<TermNode> for ChoiceNode {
    fn add_assign(&mut self, rhs: TermNode) {
        match rhs.get_choice() {
            Some(c) => c.inner.into_iter().for_each(|e| {
                self.inner.insert(e);
            }),
            None => {
                self.inner.insert(rhs);
            }
        };
    }
}
