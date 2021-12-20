use super::*;
use std::hash::{Hash, Hasher};

impl Expression {
    pub fn has_meta(&self) -> bool {
        self.node_tag.is_some()
    }
    pub fn is_choice(&self) -> bool {
        matches!(self.node, Term::Choice(_))
    }
    pub fn is_concat(&self) -> bool {
        matches!(self.node, Term::Concat(_))
    }
    pub fn is_unary(&self) -> bool {
        matches!(self.node, Term::Unary(_))
    }
}

impl Expression {
    pub fn get_concat(&self) -> Option<RefinedConcat> {
        match self.to_owned().node {
            Term::Concat(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_concat_mut(&mut self) -> Option<&mut RefinedConcat> {
        match &mut self.node {
            Term::Concat(c) => Some(c.as_mut()),
            _ => None,
        }
    }
    pub fn get_choice(&self) -> Option<ChoiceExpression> {
        match self.to_owned().node {
            Term::Choice(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_choice_mut(&mut self) -> Option<&mut ChoiceExpression> {
        match &mut self.node {
            Term::Choice(c) => Some(c.as_mut()),
            _ => None,
        }
    }
    pub fn get_unary(&self) -> Option<UnaryExpression> {
        match self.to_owned().node {
            Term::Unary(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_unary_mut(&mut self) -> Option<&mut UnaryExpression> {
        match &mut self.node {
            Term::Unary(c) => Some(c.as_mut()),
            _ => None,
        }
    }
}

impl From<Term> for Expression {
    fn from(raw: Term) -> Self {
        match raw {
            Term::Data(e) => Self::from(e),
            Term::Concat { is_soft, lhs, rhs } => match is_soft {
                true => Self::soft_concat(*lhs, *rhs),
                false => Self::concat(*lhs, *rhs),
            },
            Term::Choice { lhs, rhs } => Self::choice(*lhs, *rhs),
            Term::MarkNode { lhs, rhs } => Self::mark_node(*lhs, *rhs),
            Term::MarkNodeShort(s) => Self::mark_node(*s.clone(), *s),
            Term::MarkType { .. } => {
                unimplemented!()
            }
            Term::MustNot(_) => {
                unimplemented!()
            }
            Term::MustOne(_) => {
                unimplemented!()
            }
            Term::Maybe(e) => Self::suffix(*e, "?"),
            Term::Many(e) => Self::suffix(*e, "*"),
            Term::ManyNonNull(e) => Self::suffix(*e, "+"),
            Term::MarkBranch { base, kind, name } => Self::mark_branch(*base, kind, name),
        }
    }
}

impl From<Data> for Expression {
    fn from(e: Data) -> Self {
        Self {
            inline_token: false,
            branch_tag: None,
            ty: None,
            node_tag: None,
            node: Term::Data(box DataExpression::from(e)),
        }
    }
}

impl From<Data> for DataExpression {
    fn from(data: Data) -> Self {
        match data {
            Data::Symbol(atom) => Self::Symbol(atom),
            Data::Integer(atom) => Self::Integer(atom.data),
            Data::String(atom) => Self::String(atom.data),
            Data::Regex => unimplemented!(),
            Data::Macro(_) => unimplemented!(),
        }
    }
}

impl Hash for ChoiceExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.iter().for_each(|e| e.hash(state))
    }
}

impl Eq for DataExpression {}

impl PartialEq for DataExpression {
    fn eq(&self, other: &DataExpression) -> bool {
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

impl Hash for DataExpression {
    fn hash<H: Hasher>(&self, state: &mut H) -> () {
        match self {
            DataExpression::Symbol(e) => e.hash(state),
            DataExpression::String(e) => e.hash(state),
            DataExpression::Regex(e) => e.hash(state),
            DataExpression::Integer(e) => e.to_string().hash(state),
        }
    }
}
