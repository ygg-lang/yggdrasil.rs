use super::*;

impl ASTNode {
    #[inline]
    pub fn concat(lhs: ASTExpression, rhs: ASTExpression) -> Self {
        let mut base = ASTNode::from(lhs).as_concat();
        base.get_concat_mut().map(|e| e.bitand_assign(ASTNode::from(rhs)));
        return base;
    }
    #[inline]
    pub fn soft_concat(lhs: ASTExpression, rhs: ASTExpression) -> Self {
        let mut base = ASTNode::from(lhs).as_concat();
        base.get_concat_mut().map(|e| e.add_assign(ASTNode::from(rhs)));
        return base;
    }
    fn as_concat(self) -> Self {
        if let Some(_) = self.get_concat() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: ASTExpression::concat(self) };
    }
}

impl ASTExpression {
    pub fn concat(base: ASTNode) -> Self {
        Self::Concat(Box::new(RefinedConcat { base, rest: vec![] }))
    }
}

impl AddAssign<ASTNode> for RefinedConcat {
    /// a + b
    fn add_assign(&mut self, rhs: ASTNode) {
        match rhs.get_concat() {
            Some(c) => {
                self.rest.push((true, c.base));
                self.rest.extend(c.rest);
            }
            None => self.rest.push((true, rhs)),
        }
    }
}

impl BitAndAssign<ASTNode> for RefinedConcat {
    /// a b
    fn bitand_assign(&mut self, rhs: ASTNode) {
        match rhs.get_concat() {
            Some(c) => {
                self.rest.push((false, c.base));
                self.rest.extend(c.rest);
            }
            None => self.rest.push((false, rhs)),
        }
    }
}
