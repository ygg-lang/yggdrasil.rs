use super::*;

impl From<ChoiceExpression> for RefinedChoice {
    fn from(e: ChoiceExpression) -> Self {
        let lhs = ExpressionNode::from(e.lhs);
        let rhs = ExpressionNode::from(e.rhs);
        let mut base = Self::from(lhs);
        base += rhs;
        return base;
    }
}

impl From<ChoiceTag> for ExpressionNode {
    fn from(e: ChoiceTag) -> Self {
        ExpressionNode {
            tag: ExpressionTag::new_optional(e.tag, e.mode),
            ty: None,
            field: None,
            node: RefinedExpression::Choice(box RefinedChoice {
                inner: vec![RefinedExpression::from(e.expr)]
            })
        }
    }
}

impl From<ExpressionNode> for RefinedChoice {
    fn from(_: ExpressionNode) -> Self {
        todo!()
    }
}


impl AddAssign<ExpressionNode> for RefinedChoice {
    fn add_assign(&mut self, rhs: ExpressionNode) {
        match rhs {
            ExpressionNode::Choice(c) => self.inner.extend(c.inner),
            _ => self.inner.push(rhs),
        }
    }
}