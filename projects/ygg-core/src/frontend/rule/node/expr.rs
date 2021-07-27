use super::*;
use std::hash::{Hash, Hasher};

impl ExpressionNode {
    pub fn has_meta(&self) -> bool {
        self.field.is_some()
    }
    pub fn is_choice(&self) -> bool {
        matches!(self.node, RefinedExpression::Choice(_))
    }
    pub fn is_concat(&self) -> bool {
        matches!(self.node, RefinedExpression::Concat(_))
    }
    pub fn is_unary(&self) -> bool {
        matches!(self.node, RefinedExpression::Unary(_))
    }
}

impl ExpressionNode {
    pub fn get_concat(&self) -> Option<RefinedConcat> {
        match self.to_owned().node {
            RefinedExpression::Concat(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_choice(&self) -> Option<RefinedChoice> {
        match self.to_owned().node {
            RefinedExpression::Choice(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_unary(&self) -> Option<RefinedUnary> {
        match self.to_owned().node {
            RefinedExpression::Unary(c) => Some(*c),
            _ => None,
        }
    }
}

impl ExpressionTag {
    pub fn new(tag: Symbol, mode: Option<String>) -> Self {
        Self { tag, mode: mode.unwrap_or_default() }
    }
    pub fn new_optional(tag: Option<Symbol>, mode: Option<String>) -> Option<Self> {
        match tag {
            Some(s) => Some(Self::new(s, mode)),
            None => None,
        }
    }
}

impl From<Expression> for ExpressionNode {
    fn from(raw: Expression) -> Self {
        match raw {
            Expression::Data(e) => Self::from(e),
            Expression::Concat { is_soft, lhs, rhs } => match is_soft {
                true => Self::soft_concat(lhs, rhs),
                false => Self::concat(lhs, rhs),
            },
            Expression::Choice { lhs, rhs } => Self::choice(lhs, rhs),
            Expression::MarkNode { .. } => {
                unimplemented!()
            }
            Expression::MarkNodeShort(_) => {
                unimplemented!()
            }
            Expression::MarkType { .. } => {
                unimplemented!()
            }
            Expression::MustNot(_) => {
                unimplemented!()
            }
            Expression::MustOne(_) => {
                unimplemented!()
            }
            Expression::Maybe(_) => {
                unimplemented!()
            }
            Expression::Many(_) => {
                unimplemented!()
            }
            Expression::ManyNonNull(_) => {
                unimplemented!()
            }
            Expression::MarkBranch { .. } => {
                unimplemented!()
            }
        }
    }
}

impl From<Data> for ExpressionNode {
    fn from(e: Data) -> Self {
        Self { inline_token: false, tag: None, ty: None, field: None, node: RefinedExpression::Data(box RefinedData::from(e)) }
    }
}

impl From<Data> for RefinedData {
    fn from(data: Data) -> Self {
        match data {
            Data::Symbol(atom) => Self::Symbol(*atom),
            Data::Integer(atom) => Self::Integer(atom.data),
            Data::String(atom) => Self::String(atom.data),
            Data::Regex => unimplemented!(),
            Data::Macro => unimplemented!(),
        }
    }
}

impl Hash for RefinedChoice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.iter().for_each(|e| e.hash(state))
    }
}

impl Eq for RefinedData {}

impl PartialEq for RefinedData {
    fn eq(&self, other: &RefinedData) -> bool {
        match (self, other) {
            (Self::Symbol(lhs), Self::Symbol(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Integer(lhs), Self::Integer(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::Integer(rhs)) | (Self::Integer(rhs), Self::String(lhs)) => rhs.to_string().eq(lhs),
            (Self::Regex(lhs), Self::Regex(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Hash for RefinedData {
    fn hash<H: Hasher>(&self, state: &mut H) -> () {
        match self {
            RefinedData::Symbol(e) => e.hash(state),
            RefinedData::String(e) => e.hash(state),
            RefinedData::Regex(e) => e.hash(state),
            RefinedData::Integer(e) => e.to_string().hash(state),
        }
    }
}
