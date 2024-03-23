use crate::bootstrap::{
    AtomicNode, BooleanNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode, IdentifierNode,
    TermNode,
};

impl ExpressionNode {
    pub fn as_single(&self) -> Option<&ExpressionHardNode> {
        match self.expression_hard.as_slice() {
            [one] => Some(one),
            _ => None,
        }
    }
    pub fn as_atom(&self) -> Option<&AtomicNode> {
        self.as_single()?.as_atom()
    }
    pub fn as_boolean(&self) -> Option<bool> {
        match self.as_atom()? {
            AtomicNode::Boolean(BooleanNode::True) => Some(true),
            AtomicNode::Boolean(BooleanNode::False) => Some(false),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<&IdentifierNode> {
        if let AtomicNode::Identifier(v) = self.as_atom()? {
            return Some(v);
        }
        return None;
    }
}

impl ExpressionHardNode {
    pub fn as_single(&self) -> Option<&ExpressionSoftNode> {
        match self.expression_soft.as_slice() {
            [one] => Some(one),
            _ => None,
        }
    }

    pub fn as_atom(&self) -> Option<&AtomicNode> {
        self.as_single()?.as_single()?.as_single()?.as_single()
    }
    pub fn as_identifier(&self) -> Option<&IdentifierNode> {
        self.as_atom()?.as_identifier()
    }
}

impl ExpressionSoftNode {
    pub fn as_single(&self) -> Option<&ExpressionTagNode> {
        match self.expression_tag.as_slice() {
            [one] => Some(one),
            _ => None,
        }
    }
}
impl ExpressionTagNode {
    pub fn as_single(&self) -> Option<&TermNode> {
        match self.identifier.as_ref() {
            Some(_) => None,
            _ => Some(&self.term),
        }
    }
}

impl TermNode {
    pub fn as_single(&self) -> Option<&AtomicNode> {
        if self.prefix.is_empty() && self.suffix.is_empty() {
            return Some(&self.atomic);
        }
        return None;
    }
}
impl AtomicNode {
    pub fn as_identifier(&self) -> Option<&IdentifierNode> {
        match self {
            Self::Identifier(s) => Some(s),
            _ => None,
        }
    }
}
