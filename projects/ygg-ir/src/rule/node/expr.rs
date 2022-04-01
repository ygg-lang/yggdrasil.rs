use super::*;

impl ExpressionNode {
    pub fn is_choice(&self) -> bool {
        matches!(self, ExpressionNode::Choice(_))
    }
    pub fn is_concat(&self) -> bool {
        matches!(self, ExpressionNode::Concat(_))
    }
    pub fn is_unary(&self) -> bool {
        matches!(self, ExpressionNode::Unary(_))
    }
}

impl ExpressionNode {
    pub fn set_tag(&mut self, tag: String) {
        match self {
            ExpressionNode::Choice(_) => {}
            ExpressionNode::Concat(_) => {}
            ExpressionNode::Unary(e) => e.set_tag(tag),
            ExpressionNode::Data(e) => e.set_tag(tag),
        }
    }
}
