use super::*;

impl ExpressionNode {
    pub fn is_choice(&self) -> bool {
        matches!(self.kind, ExpressionKind::Choice(_))
    }
    pub fn is_concat(&self) -> bool {
        matches!(self.kind, ExpressionKind::Concat(_))
    }
    pub fn is_unary(&self) -> bool {
        matches!(self.kind, ExpressionKind::Unary(_))
    }
}

impl ExpressionKind {
    pub fn as_tag(&self) -> Option<&str> {
        match self {
            ExpressionKind::Rule(r) => Some(&r.name),
            _ => None,
        }
    }
}
