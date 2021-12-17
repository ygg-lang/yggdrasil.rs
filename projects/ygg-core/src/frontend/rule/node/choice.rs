use super::*;

impl ASTNode {
    #[inline]
    pub fn choice(lhs: ASTExpression, rhs: ASTExpression) -> Self {
        let mut base = ASTNode::from(lhs).as_choice();
        base.get_choice_mut().map(|e| e.add_assign(ASTNode::from(rhs)));
        return base;
    }
    fn as_choice(self) -> Self {
        if let Some(_) = self.get_choice() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: ASTExpression::concat(self) };
    }
}

impl ASTExpression {
    pub fn choice(base: ASTNode) -> Self {
        let mut inner = IndexSet::default();
        inner.insert(base);
        Self::Choice(Box::new(RefinedChoice { inner }))
    }
}

impl From<ASTExpression> for RefinedChoice {
    fn from(e: ASTExpression) -> Self {
        Self::from(ASTNode::from(e))
    }
}

impl From<ASTNode> for RefinedChoice {
    fn from(e: ASTNode) -> Self {
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

impl AddAssign<ASTNode> for RefinedChoice {
    fn add_assign(&mut self, rhs: ASTNode) {
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
