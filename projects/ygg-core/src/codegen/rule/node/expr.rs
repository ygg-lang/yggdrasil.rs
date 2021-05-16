use std::hash::{Hash, Hasher};
use super::*;

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
    pub fn new(tag: Identifier, mode: Option<String>) -> Self {
        Self { tag, mode: mode.unwrap_or_default() }
    }
    pub fn new_optional(tag: Option<Identifier>, mode: Option<String>) -> Option<Self> {
        match tag {
            Some(s) => Some(Self::new(s, mode)),
            None => None,
        }
    }
}

impl From<Expression> for ExpressionNode {
    fn from(raw: Expression) -> Self {
        match raw {
            Expression::Data(e) => Self::from(*e),
            Expression::UnarySuffix(e) => Self::from(*e),
            Expression::UnaryPrefix(e) => Self::from(*e),
            Expression::ConcatExpression(e) => Self::from(*e),
            Expression::ChoiceExpression(e) => Self::from(*e),
            Expression::FieldExpression(e) => Self::from(*e),
        }
    }
}

impl From<FieldExpression> for ExpressionNode {
    fn from(e: FieldExpression) -> Self {
        Self {
            inline_token: false,
            tag: None,
            ty: None,
            field: Some(e.lhs),
            node: Self::from(e.rhs).node
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
            Data::Identifier(atom) => Self::Identifier(*atom),
            Data::Integer(atom) => Self::Integer(atom.data),
            Data::String(atom) => Self::String(atom.data),
            Data::Regex => Self::Integer(0),
            Data::Macro => Self::Integer(0),
        }
    }
}

impl Hash for RefinedChoice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.iter().for_each(|e| e.hash(state))
    }
}

impl Eq for RefinedData {

}
impl PartialEq for RefinedData {
    fn eq(&self, other: &RefinedData) -> bool {
        match (self, other) {
            (Self::Identifier(lhs) , Self::Identifier(rhs)) => {
                lhs == rhs
            }

            (Self::Regex(lhs) , Self::Regex(rhs)) => {
                lhs == rhs
            }
            (Self::String(lhs) , Self::String(rhs)) => {
                lhs == rhs
            }
            (Self::Integer(lhs) , Self::Integer(rhs)) => {
                lhs == rhs
            }
            (Self::String(lhs) , Self::Integer(rhs))
            |(Self::Integer(rhs) , Self::String(lhs))=> {
                rhs.to_string().eq(lhs)
            }
            _=>{
                false
            }
        }
    }
}

impl Hash for RefinedData {
    fn hash<H: Hasher>(&self, state: &mut H) -> () {
        match self {
            RefinedData::Identifier(e) => {e.hash(state)}
            RefinedData::String(e) => {e.hash(state)}
            RefinedData::Regex(e) => {e.hash(state)}
            RefinedData::Integer(e) => {e.to_string().hash(state)}
        }
    }
}