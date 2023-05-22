use crate::bootstrap::{AtomicNode, ExpressionNode, IdentifierNode};

impl ExpressionNode {
    pub fn as_identifier(&self) -> Option<&IdentifierNode> {
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
            if let AtomicNode::Identifier(v) = &expr.atomic {
                return Some(v);
            }
        }
        return None;
    }
}
