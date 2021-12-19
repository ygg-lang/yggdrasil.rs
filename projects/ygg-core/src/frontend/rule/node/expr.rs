use super::*;
use std::hash::{Hash, Hasher};

impl TermNode {
    pub fn has_meta(&self) -> bool {
        self.node_tag.is_some()
    }
    pub fn is_choice(&self) -> bool {
        matches!(self.node, ExpressionNode::Choice(_))
    }
    pub fn is_concat(&self) -> bool {
        matches!(self.node, ExpressionNode::Concat(_))
    }
    pub fn is_unary(&self) -> bool {
        matches!(self.node, ExpressionNode::Unary(_))
    }
}

impl TermNode {
    pub fn get_concat(&self) -> Option<RefinedConcat> {
        match self.to_owned().node {
            ExpressionNode::Concat(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_concat_mut(&mut self) -> Option<&mut RefinedConcat> {
        match &mut self.node {
            ExpressionNode::Concat(c) => Some(c.as_mut()),
            _ => None,
        }
    }
    pub fn get_choice(&self) -> Option<ChoiceNode> {
        match self.to_owned().node {
            ExpressionNode::Choice(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_choice_mut(&mut self) -> Option<&mut ChoiceNode> {
        match &mut self.node {
            ExpressionNode::Choice(c) => Some(c.as_mut()),
            _ => None,
        }
    }
    pub fn get_unary(&self) -> Option<RefinedUnary> {
        match self.to_owned().node {
            ExpressionNode::Unary(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_unary_mut(&mut self) -> Option<&mut RefinedUnary> {
        match &mut self.node {
            ExpressionNode::Unary(c) => Some(c.as_mut()),
            _ => None,
        }
    }
}

impl From<ExpressionNode> for TermNode {
    fn from(raw: ExpressionNode) -> Self {
        match raw {
            ExpressionNode::Data(e) => Self::from(e),
            ExpressionNode::Concat { is_soft, lhs, rhs } => match is_soft {
                true => Self::soft_concat(*lhs, *rhs),
                false => Self::concat(*lhs, *rhs),
            },
            ExpressionNode::Choice { lhs, rhs } => Self::choice(*lhs, *rhs),
            ExpressionNode::MarkNode { lhs, rhs } => Self::mark_node(*lhs, *rhs),
            ExpressionNode::MarkNodeShort(s) => Self::mark_node(*s.clone(), *s),
            ExpressionNode::MarkType { .. } => {
                unimplemented!()
            }
            ExpressionNode::MustNot(_) => {
                unimplemented!()
            }
            ExpressionNode::MustOne(_) => {
                unimplemented!()
            }
            ExpressionNode::Maybe(e) => Self::suffix(*e, "?"),
            ExpressionNode::Many(e) => Self::suffix(*e, "*"),
            ExpressionNode::ManyNonNull(e) => Self::suffix(*e, "+"),
            ExpressionNode::MarkBranch { base, kind, name } => Self::mark_branch(*base, kind, name),
        }
    }
}

impl From<Data> for TermNode {
    fn from(e: Data) -> Self {
        Self {
            inline_token: false,
            branch_tag: None,
            ty: None,
            node_tag: None,
            node: ExpressionNode::Data(box DataNode::from(e)),
        }
    }
}

impl From<Data> for DataNode {
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

impl Hash for ChoiceNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.iter().for_each(|e| e.hash(state))
    }
}

impl Eq for DataNode {}

impl PartialEq for DataNode {
    fn eq(&self, other: &DataNode) -> bool {
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

impl Hash for DataNode {
    fn hash<H: Hasher>(&self, state: &mut H) -> () {
        match self {
            DataNode::Symbol(e) => e.hash(state),
            DataNode::String(e) => e.hash(state),
            DataNode::Regex(e) => e.hash(state),
            DataNode::Integer(e) => e.to_string().hash(state),
        }
    }
}
