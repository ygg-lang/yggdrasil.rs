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
    pub fn empty() -> Self {
        Self { kind: ExpressionKind::Choice(Box::new(Default::default())), branch_tag: "".to_string(), node_tag: "".to_string() }
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
