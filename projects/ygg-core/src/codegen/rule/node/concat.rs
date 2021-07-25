use super::*;

impl ExpressionNode {
    #[inline]
    pub fn concat(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self { inline_token: false, tag: None, ty: None, field: None, node: RefinedExpression::concat(lhs, rhs) }
    }
    #[inline]
    pub fn soft_concat(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self { inline_token: false, tag: None, ty: None, field: None, node: RefinedExpression::soft_concat(lhs, rhs) }
    }
}

impl RefinedExpression {
    pub fn concat(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        unimplemented!()
    }
    pub fn soft_concat(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        unimplemented!()
    }
}

impl From<ExpressionNode> for RefinedConcat {
    fn from(e: ExpressionNode) -> Self {
        match e.get_concat() {
            Some(s) => Self { base: s.base, rest: vec![] },
            None => Self { base: e, rest: vec![] },
        }
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
