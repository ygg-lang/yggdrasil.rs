use crate::bootstrap::{
    AtomicNode, BooleanNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode, IdentifierNode,
    TermNode,
};

impl<'i> ExpressionNode<'i> {
    pub fn to_single(self) -> Option<ExpressionHardNode<'i>> {
        let mut children = self.expression_hard();
        if children.len() == 1 { children.pop() } else { None }
    }
    pub fn to_atom(self) -> Option<AtomicNode<'i>> {
        self.to_single()?.to_atom()
    }
    pub fn to_boolean(self) -> Option<bool> {
        match self.to_atom()? {
            AtomicNode::Boolean(BooleanNode::True(_)) => Some(true),
            AtomicNode::Boolean(BooleanNode::False(_)) => Some(false),
            _ => None,
        }
    }
    pub fn to_identifier(self) -> Option<IdentifierNode<'i>> {
        if let AtomicNode::Identifier(v) = self.to_atom()? {
            return Some(v);
        }
        return None;
    }
}

impl<'i> ExpressionHardNode<'i> {
    pub fn to_single(self) -> Option<ExpressionSoftNode<'i>> {
        let mut children = self.expression_soft();
        if children.len() == 1 { return children.pop() } else { None }
    }

    pub fn to_atom(self) -> Option<AtomicNode<'i>> {
        self.to_single()?.to_single()?.to_single()?.to_single()
    }
    pub fn to_identifier(self) -> Option<IdentifierNode<'i>> {
        self.to_atom()?.to_identifier()
    }
}

impl<'i> ExpressionSoftNode<'i> {
    pub fn to_single(self) -> Option<ExpressionTagNode<'i>> {
        let mut children = self.expression_tag();
        if children.len() == 1 { return children.pop() } else { None }
    }
}
impl<'i> ExpressionTagNode<'i> {
    pub fn to_single(self) -> Option<TermNode<'i>> {
        match self.identifier() {
            Some(_) => None,
            _ => Some(self.term()),
        }
    }
}

impl<'i> TermNode<'i> {
    pub fn to_single(self) -> Option<AtomicNode<'i>> {
        if self.prefix().is_empty() && self.suffix().is_empty() {
            return Some(self.atomic());
        }
        return None;
    }
}
impl<'i> AtomicNode<'i> {
    pub fn to_identifier(self) -> Option<IdentifierNode<'i>> {
        match self {
            Self::Identifier(s) => Some(s),
            _ => None,
        }
    }
}
