use super::*;

impl From<ChoiceExpression> for ExpressionNode {
    fn from(e: ChoiceExpression) -> Self {
        Self { tag: None, ty: None, field: None, node: RefinedExpression::Choice(box RefinedChoice::from(e)) }
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
            None => Self { inner: vec![e] },
        }
    }
}

impl From<ChoiceTag> for ExpressionNode {
    fn from(e: ChoiceTag) -> Self {
        ExpressionNode {
            tag: ExpressionTag::new_optional(e.tag, e.mode),
            ty: e.ty,
            field: None,
            node: RefinedExpression::Choice(box RefinedChoice { inner: vec![ExpressionNode::from(e.expr)] }),
        }
    }
}

impl From<ChoiceTag> for RefinedChoice {
    fn from(e: ChoiceTag) -> Self {
        Self {
            inner: vec![ExpressionNode::from(e)]
        }
    }
}

impl AddAssign<ExpressionNode> for RefinedChoice {
    fn add_assign(&mut self, rhs: ExpressionNode) {
        match rhs.get_choice() {
            Some(c) => self.inner.extend(c.inner),
            None => self.inner.push(rhs),
        }
    }
}
