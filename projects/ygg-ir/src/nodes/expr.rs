use super::*;
use crate::data::RuleReference;

impl YggdrasilExpression {
    #[inline]
    pub fn is_choice(&self) -> bool {
        matches!(self.kind, ExpressionKind::Choice(_))
    }
    #[inline]
    pub fn is_concat(&self) -> bool {
        matches!(self.kind, ExpressionKind::Concat(_))
    }
    #[inline]
    pub fn is_unary(&self) -> bool {
        matches!(self.kind, ExpressionKind::Unary(_))
    }
    #[inline]
    pub fn as_rule(&self) -> Option<&RuleReference> {
        match &self.kind {
            ExpressionKind::Rule(r) => Some(r),
            _ => None,
        }
    }
    #[inline]
    pub fn is_rule(&self) -> bool {
        self.as_rule().is_some()
    }
    #[inline]
    pub fn with_tag(self, tag: YggdrasilIdentifier) -> Self {
        Self { tag: Some(tag), ..self }
    }
    #[inline]
    pub fn with_remark(self) -> Self {
        Self { remark: true, ..self }
    }
}

impl ExpressionKind {
    pub fn as_tag(&self) -> Option<&str> {
        self.as_rule().map(|r| r.name.text.as_str())
    }
    pub fn as_rule(&self) -> Option<&RuleReference> {
        match self {
            ExpressionKind::Rule(r) => Some(r),
            _ => None,
        }
    }
}
