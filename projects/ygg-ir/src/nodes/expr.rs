use super::*;
use crate::data::RuleReference;
use yggdrasil_parser::bootstrap::IdentifierNode;

impl YggdrasilExpression {
    #[inline]
    pub fn is_choice(&self) -> bool {
        matches!(self.body, ExpressionBody::Choice(_))
    }
    #[inline]
    pub fn is_concat(&self) -> bool {
        matches!(self.body, ExpressionBody::Concat(_))
    }
    #[inline]
    pub fn is_unary(&self) -> bool {
        matches!(self.body, ExpressionBody::Unary(_))
    }
    #[inline]
    pub fn as_rule(&self) -> Option<&RuleReference> {
        match &self.body {
            ExpressionBody::Rule(r) => Some(r),
            _ => None,
        }
    }
    #[inline]
    pub fn as_identifier(&self) -> Option<&YggdrasilIdentifier> {
        Some(&self.as_rule()?.name)
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

impl ExpressionBody {
    pub fn as_tag(&self) -> Option<&str> {
        self.as_rule().map(|r| r.name.text.as_str())
    }
    pub fn as_rule(&self) -> Option<&RuleReference> {
        match self {
            ExpressionBody::Rule(r) => Some(r),
            _ => None,
        }
    }
}
