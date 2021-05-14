use super::*;

impl From<ConcatExpression> for ExpressionNode {
    fn from(e: ConcatExpression) -> Self {
        Self { inline_token: false, tag: None, ty: None, field: None, node: RefinedExpression::Concat(box RefinedConcat::from(e)) }
    }
}

impl From<ConcatExpression> for RefinedConcat {
    fn from(e: ConcatExpression) -> Self {
        let lhs = ExpressionNode::from(e.lhs);
        let rhs = ExpressionNode::from(e.rhs);
        let mut base = Self::from(lhs);
        base += rhs;
        return base;
    }
}

impl From<ExpressionNode> for RefinedConcat {
    fn from(e: ExpressionNode) -> Self {
        match e.get_concat() {
            Some(s) => Self { inner: s.inner },
            None => Self { inner: vec![e] },
        }
    }
}

impl AddAssign<ExpressionNode> for RefinedConcat {
    fn add_assign(&mut self, rhs: ExpressionNode) {
        match rhs.get_concat() {
            Some(c) => self.inner.extend(c.inner),
            None => self.inner.push(rhs),
        }
    }
}
