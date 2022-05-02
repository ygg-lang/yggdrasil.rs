use super::*;
use convert_case::{Case, Casing};

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
        Self { kind: ExpressionKind::Choice(Box::new(Default::default())), tag: "".to_string() }
    }
    pub fn ignored() -> Self {
        Self { kind: ExpressionKind::Data(Box::new(DataKind::Ignored)), tag: "".to_string() }
    }
    pub fn remark(&mut self, capture: bool) -> Result<(), YggdrasilError> {
        let rule = match self.kind.as_rule() {
            Some(r) => r,
            None => return Err(YggdrasilError::language_error("can't remark")),
        };
        match capture {
            true => self.tag = rule.name.to_case(Case::Snake),
            false => self.tag = String::new(),
        }
        Ok(())
    }
}

impl ExpressionKind {
    pub fn as_tag(&self) -> Option<&str> {
        self.as_rule().map(|r| r.name.as_str())
    }
    pub fn as_rule(&self) -> Option<&RuleReference> {
        match self {
            ExpressionKind::Rule(r) => Some(r),
            _ => None,
        }
    }
}

impl BitXor<Self> for ExpressionNode {
    type Output = Result<Self, YggdrasilError>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self.kind.as_tag() {
            Some(s) => {
                let node_tag = match s {
                    "_" => "".to_string(),
                    _ => s.to_string(),
                };
                Ok(ExpressionNode { kind: rhs.kind, tag: node_tag })
            }
            None => Err(YggdrasilError::language_error("lhs not a valid tag")),
        }
    }
}
