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
    pub fn with_tag(mut self, tag: YggdrasilIdentifier) -> Self {
        self.tag = Some(tag);
        self
    }
    pub fn remark(&mut self, capture: bool) {
        match capture {
            true => self.capture(),
            false => self.non_capture(),
        }
    }
    fn capture(&mut self) {
        match &mut self.kind {
            ExpressionKind::Choice(e) => {
                todo!()
            }
            ExpressionKind::Concat(e) => todo!(),
            ExpressionKind::Unary(e) => match e.operators.contains(&Operator::Remark) {
                true => e.base.non_capture(),
                false => e.base.capture(),
            },
            ExpressionKind::Rule(e) => {
                todo!();
                // self.tag = e.name.to_case(Case::Snake)
            }
            ExpressionKind::Function(_) => {
                self.tag.take();
            }
            _ => {
                self.tag.take();
            }
        }
    }
    fn non_capture(&mut self) {
        match &mut self.kind {
            ExpressionKind::Choice(e) => {
                todo!()
            }
            ExpressionKind::Concat(e) => todo!(),
            ExpressionKind::Unary(e) => match e.operators.contains(&Operator::Remark) {
                true => e.base.capture(),
                false => e.base.non_capture(),
            },
            ExpressionKind::Function(_) => {
                self.tag.take();
            }
            ExpressionKind::Rule(_) => {
                self.tag.take();
            }
            ExpressionKind::Data(_) => {
                self.tag.take();
            }
            ExpressionKind::Regex(_) => {
                self.tag.take();
            }
            _ => {
                self.tag.take();
            }
        }
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

impl BitXor<Self> for YggdrasilExpression {
    type Output = QResult<Self>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
