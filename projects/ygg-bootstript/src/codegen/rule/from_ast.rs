use super::*;
use convert_case::{Case, Casing};

use crate::ast::{AssignStatement, ChoiceExpression, Expression};
use crate::ygg::ast::ConcatExpression;

impl From<AssignStatement> for YGGRule {
    fn from(s: AssignStatement) -> Self {
        let mut name = s.id.data;
        let mut structure = None;
        let force_inline = name.starts_with("_");
        match force_inline {
            true => name = String::from(&name[1..=name.len()]),
            false => structure = Some(name.to_case(Case::UpperCamel)),
        }
        let mut eliminate_unmarked = false;
        let mut eliminate_unnamed = false;
        match s.eq.as_str() {
            "_=" => eliminate_unnamed = true,
            "^=" => eliminate_unmarked = true,
            _ => unreachable!(),
        }
        let expression = RefinedExpression::from(s.rhs);

        Self { name, structure_name: structure, force_inline, eliminate_unmarked, eliminate_unnamed, expression }
    }
}

impl From<Expression> for RefinedExpression {
    fn from(raw: Expression) -> Self {
        match raw {
            Expression::Data(_) => { unimplemented!() }
            Expression::UnarySuffix(_) => { unimplemented!() }
            Expression::UnaryPrefix(_) => { unimplemented!() }
            Expression::ConcatExpression(c) => {
                Self::Concat(box RefinedConcat::from(*c))
            }
            Expression::ChoiceExpression(_) => {
                unimplemented!()
            }
            Expression::FieldExpression(_) => { unimplemented!() }
        }
    }
}

impl From<ConcatExpression> for RefinedConcat {
    fn from(e: ConcatExpression) -> Self {
        let lhs = Self::from(e.lhs);
        let rhs = Self::from(e.rhs);
        return lhs + rhs
    }
}

impl From<Expression> for RefinedConcat {
    fn from(e: Expression) -> Self {
        match e {
            Expression::Data(_) => {unimplemented!()}
            Expression::UnarySuffix(_) => {unimplemented!()}
            Expression::UnaryPrefix(_) => {unimplemented!()}
            Expression::ConcatExpression(_) => {unimplemented!()}
            Expression::ChoiceExpression(_) => {unimplemented!()}
            Expression::FieldExpression(_) => {unimplemented!()}
        }
    }
}
