use super::*;
use crate::data::RuleReference;

impl ExpressionNode {
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
    pub fn empty() -> Self {
        Self { kind: ExpressionKind::Choice(Box::new(Default::default())), tag: "".to_string() }
    }
    pub fn character(c: char) -> Self {
        Self { kind: ExpressionKind::Data(Box::new(DataKind::Character(c))), tag: "".to_string() }
    }
    pub fn ignored() -> Self {
        Self { kind: ExpressionKind::Ignored, tag: "".to_string() }
    }
    pub fn with_tag<S>(mut self, tag: S) -> Self
    where
        S: Into<String>,
    {
        self.tag = tag.into();
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
                for mut node in take(&mut e.branches) {
                    node.capture();
                    e.branches.insert(node);
                }
            }
            ExpressionKind::Concat(e) => e.into_iter().for_each(|f| f.capture()),
            ExpressionKind::Unary(e) => match e.operators.contains(&Operator::Remark) {
                true => e.base.non_capture(),
                false => e.base.capture(),
            },
            ExpressionKind::Rule(e) => {
                todo!();
                // self.tag = e.name.to_case(Case::Snake)
            }
            ExpressionKind::Function(_) => self.tag.clear(),
            ExpressionKind::Data(_) => self.tag.clear(),
            ExpressionKind::Regex(_) => self.tag.clear(),
            ExpressionKind::Text(_) => {}
            ExpressionKind::Ignored => {}
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
            ExpressionKind::Concat(e) => e.into_iter().for_each(|f| f.non_capture()),
            ExpressionKind::Unary(e) => match e.operators.contains(&Operator::Remark) {
                true => e.base.capture(),
                false => e.base.non_capture(),
            },
            ExpressionKind::Function(_) => self.tag.clear(),
            ExpressionKind::Rule(_) => self.tag.clear(),
            ExpressionKind::Data(_) => self.tag.clear(),
            ExpressionKind::Regex(_) => self.tag.clear(),
            ExpressionKind::Text(_) => {}
            ExpressionKind::Ignored => {}
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
