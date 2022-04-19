use super::*;
use std::ops::Div;
use yggdrasil_error::YggdrasilError;

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

impl Div<Self> for ExpressionNode {
    type Output = Result<Self, YggdrasilError>;
    fn div(self, rhs: Self) -> Self::Output {
        match self.kind.as_tag() {
            Some(s) => {
                let node_tag = match s {
                    "_" => "".to_string(),
                    _ => s.to_string(),
                };
                Ok(ExpressionNode { kind: rhs.kind, branch_tag: rhs.branch_tag, node_tag })
            }
            None => Err(YggdrasilError::language_error("lhs not a valid tag")),
        }
    }
}
