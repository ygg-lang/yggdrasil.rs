use super::*;

impl ExpressionKind {
    pub fn is_choice(&self) -> bool {
        matches!(self, ExpressionKind::Choice(_))
    }
    pub fn is_concat(&self) -> bool {
        matches!(self, ExpressionKind::Concat(_))
    }
    pub fn is_unary(&self) -> bool {
        matches!(self, ExpressionKind::Unary(_))
    }
}

impl ExpressionKind {
    pub fn set_tag(&mut self, tag: String) {
        match self {
            ExpressionKind::Choice(_) => {}
            ExpressionKind::Concat(_) => {}
            ExpressionKind::Unary(e) => e.set_tag(tag),
            ExpressionKind::Data(e) => e.set_tag(tag),
        }
    }
}
