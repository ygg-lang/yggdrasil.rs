use super::*;

impl ExpressionNode {
    #[inline]
    pub fn concat(lhs: Expression, rhs: Expression) -> Self {
        let mut base = ExpressionNode::from(lhs).as_concat();
        base.get_concat_mut().map(|e|e.add_assign( ExpressionNode::from(rhs)));
        return base
    }
    #[inline]
    pub fn soft_concat(lhs: Expression, rhs: Expression) -> Self {
        let mut base = ExpressionNode::from(lhs).as_concat();
        base.get_concat_mut().map(|e|e.bitand_assign( ExpressionNode::from(rhs)));
        return base
    }
    fn as_concat(self) -> Self {
        if let Some(_) = self.get_concat() { return self }
        return  Self {
            inline_token: false,
            ty: None,
            branch_tag: None,
            node_tag: None,
            node: RefinedExpression::concat(self)
        }
    }
}

impl RefinedExpression {
    pub fn concat(base: ExpressionNode) -> Self {
        Self::Concat(Box::new(RefinedConcat{ base, rest: vec![] }))
    }
}

impl AddAssign<ExpressionNode> for RefinedConcat {
    /// a + b
    fn add_assign(&mut self, rhs: ExpressionNode) {
        match rhs.get_concat() {
            Some(c) => {
                self.rest.push((true, c.base));
                self.rest.extend(c.rest);
            }
            None => self.rest.push((true, rhs)),
        }
    }
}

impl BitAndAssign<ExpressionNode> for RefinedConcat {
    /// a b
    fn bitand_assign(&mut self, rhs: ExpressionNode) {
        match rhs.get_concat() {
            Some(c) => {
                self.rest.push((false, c.base));
                self.rest.extend(c.rest);
            }
            None => self.rest.push((false, rhs)),
        }
    }
}
