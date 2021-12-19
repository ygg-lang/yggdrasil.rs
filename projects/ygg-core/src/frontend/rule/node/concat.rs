use super::*;

impl TermNode {
    #[inline]
    pub fn concat(lhs: ExpressionNode, rhs: ExpressionNode) -> Self {
        let mut base = TermNode::from(lhs).as_concat();
        base.get_concat_mut().map(|e| e.bitand_assign(TermNode::from(rhs)));
        return base;
    }
    #[inline]
    pub fn soft_concat(lhs: ExpressionNode, rhs: ExpressionNode) -> Self {
        let mut base = TermNode::from(lhs).as_concat();
        base.get_concat_mut().map(|e| e.add_assign(TermNode::from(rhs)));
        return base;
    }
    fn as_concat(self) -> Self {
        if let Some(_) = self.get_concat() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: ExpressionNode::concat(self) };
    }
}

impl ExpressionNode {
    pub fn concat(base: TermNode) -> Self {
        Self::Concat(Box::new(RefinedConcat { base, rest: vec![] }))
    }
}

impl AddAssign<TermNode> for RefinedConcat {
    /// a + b
    fn add_assign(&mut self, rhs: TermNode) {
        match rhs.get_concat() {
            Some(c) => {
                self.rest.push((true, c.base));
                self.rest.extend(c.rest);
            }
            None => self.rest.push((true, rhs)),
        }
    }
}

impl BitAndAssign<TermNode> for RefinedConcat {
    /// a b
    fn bitand_assign(&mut self, rhs: TermNode) {
        match rhs.get_concat() {
            Some(c) => {
                self.rest.push((false, c.base));
                self.rest.extend(c.rest);
            }
            None => self.rest.push((false, rhs)),
        }
    }
}
