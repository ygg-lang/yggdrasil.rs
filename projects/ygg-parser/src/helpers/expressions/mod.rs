use crate::bootstrap::{AtomicNode, BooleanNode, ExpressionNode, IdentifierNode};

impl ExpressionNode {
    pub fn as_atom(&self) -> Option<&AtomicNode> {
        if self.expression_hard.len() != 1 {
            return None;
        }
        let expr = self.expression_hard.first()?;
        if expr.expression_soft.len() != 1 {
            return None;
        }
        let expr = expr.expression_soft.first()?;
        if expr.expression_tag.len() != 1 {
            return None;
        }
        let expr = expr.expression_tag.first()?;
        if expr.identifier.is_some() {
            return None;
        }
        let expr = &expr.term;
        if expr.prefix.is_empty() && expr.suffix.is_empty() {
            return Some(&expr.atomic);
        }
        return None;
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
