use super::*;

impl ExpressionNode {
    #[inline]
    pub fn choice(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self { inline_token: false, branch_tag: None, ty: None, node_tag: None, node: RefinedExpression::choice(lhs, rhs) }
    }
}

impl RefinedExpression {
    pub fn choice(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        let mut base = RefinedChoice::from(*lhs);
        base += *rhs;
        unimplemented!("{:#?}", base);
        Self::Choice(Box::new(base))
    }
}

impl From<Expression> for RefinedChoice {
    fn from(e: Expression) -> Self {
        Self::from(ExpressionNode::from(e))
    }
}

impl From<ExpressionNode> for RefinedChoice {
    fn from(e: ExpressionNode) -> Self {
        match e.get_choice() {
            Some(s) => Self { inner: s.inner },
            None => {
                let mut inner = Set::new();
                inner.insert(e);
                Self { inner }
            }
        }
    }
}

impl AddAssign<ExpressionNode> for RefinedChoice {
    fn add_assign(&mut self, rhs: ExpressionNode) {
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

impl AddAssign<Expression> for RefinedChoice {
    fn add_assign(&mut self, rhs: Expression) {
        self.add_assign(ExpressionNode::from(rhs))
    }
}
