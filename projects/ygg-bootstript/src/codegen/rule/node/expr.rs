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
            Expression::FieldExpression(_) => {
                unimplemented!()
            }
        }
    }
}

impl From<Data> for ExpressionNode {
    fn from(e: Data) -> Self {
        Self { tag: None, ty: None, field: None, node: RefinedExpression::Data(box RefinedData::from(e)) }
    }
}

impl From<Data> for RefinedData {
    fn from(data: Data) -> Self {
        match data {
            Data::Identifier(atom) => Self::Identifier(*atom),
            Data::Integer(atom) => Self::Integer(atom.data),
            Data::String(atom) => Self::String(atom.data),
            Data::Regex => {
                unimplemented!()
            }
        }
    }
}
