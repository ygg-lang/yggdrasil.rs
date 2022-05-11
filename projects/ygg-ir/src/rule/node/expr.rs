use convert_case::{Case, Casing};
use diagnostic_quick::{QError, QResult};

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
        Self { kind: ExpressionKind::Choice(Box::new(Default::default())), tag: "".to_string() }
    }
    pub fn ignored() -> Self {
        Self { kind: ExpressionKind::Data(Box::new(DataKind::Ignored)), tag: "".to_string() }
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
                for mut node in take(&mut e.branches) {
                    node.capture();
                    e.branches.insert(node);
                }
            }
            ExpressionKind::Concat(e) => e.sequence.iter_mut().for_each(|f| f.capture()),
            ExpressionKind::Unary(e) => match e.ops.contains(&Operator::Remark) {
                true => e.base.non_capture(),
                false => e.base.capture(),
            },
            ExpressionKind::Rule(e) => self.tag = e.name.to_case(Case::Snake),
            ExpressionKind::Function(_) => self.tag.clear(),
            ExpressionKind::Data(_) => self.tag.clear(),
        }
    }
    fn non_capture(&mut self) {
        match &mut self.kind {
            ExpressionKind::Choice(e) => {
                for mut node in take(&mut e.branches) {
                    node.non_capture();
                    e.branches.insert(node);
                }
            }
            ExpressionKind::Concat(e) => e.sequence.iter_mut().for_each(|f| f.non_capture()),
            ExpressionKind::Unary(e) => match e.ops.contains(&Operator::Remark) {
                true => e.base.capture(),
                false => e.base.non_capture(),
            },
            ExpressionKind::Function(_) => self.tag.clear(),
            ExpressionKind::Rule(_) => self.tag.clear(),
            ExpressionKind::Data(_) => self.tag.clear(),
        }
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
    type Output = QResult<Self>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self.kind.as_tag() {
            Some(s) => {
                let node_tag = match s {
                    "_" => "".to_string(),
                    _ => s.to_string(),
                };
                Ok(ExpressionNode { kind: rhs.kind, tag: node_tag })
            }
            None => Err(QError::runtime_error("lhs not a valid tag")),
        }
    }
}
