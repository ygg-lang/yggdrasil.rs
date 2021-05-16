use super::*;

impl From<ChoiceExpression> for ExpressionNode {
    fn from(e: ChoiceExpression) -> Self {
        Self {
            inline_token: false,
            tag: None,
            ty: None,
            field: None,
            node: RefinedExpression::Choice(box RefinedChoice::from(e)),
        }
    }
}

impl From<ChoiceExpression> for RefinedChoice {
    fn from(e: ChoiceExpression) -> Self {
        let lhs = ExpressionNode::from(e.lhs);
        let rhs = ExpressionNode::from(e.rhs);
        let mut base = Self::from(lhs);
        base += rhs;
        return base;
    }
}

impl From<ExpressionNode> for RefinedChoice {
    fn from(e: ExpressionNode) -> Self {
        match e.get_choice() {
            Some(s) => Self { inner: s.inner },
            None => {
                let mut inner = Set::new();
                inner.insert(e);
                Self { inner } },
        }
    }
}

impl From<ChoiceTag> for ExpressionNode {
    fn from(e: ChoiceTag) -> Self {
        let mut inner = Set::new();
        inner.insert(ExpressionNode::from(e.expr));
        ExpressionNode {
            inline_token: false,
            tag: ExpressionTag::new_optional(e.tag, e.mode),
            ty: e.ty,
            field: None,
            node: RefinedExpression::Choice(box RefinedChoice { inner }),
        }
    }
}

impl From<ChoiceTag> for RefinedChoice {
    fn from(e: ChoiceTag) -> Self {
        let mut inner = Set::new();
        inner.insert(ExpressionNode::from(e));
        Self { inner }
    }
}

impl AddAssign<ExpressionNode> for RefinedChoice {
    fn add_assign(&mut self, rhs: ExpressionNode) {
        match rhs.get_choice() {
            Some(c) => {
                c.inner.into_iter().for_each(|e|{self.inner.insert(e);})
            },
            None => {
                self.inner.insert(rhs);
            },
        }
    }
}
