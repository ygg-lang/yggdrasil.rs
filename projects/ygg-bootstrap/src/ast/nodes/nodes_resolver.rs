use super::*;
use yggdrasil_shared::position_system::join_position;

impl Data {
    pub fn position(&self) -> OffsetRange {
        match self {
            Data::SymbolPath(v) => v.position,
            Data::Integer(v) => v.position,
            Data::String(v) => v.position,
            Data::Macro => {
                unimplemented!()
            }
            Data::Regex => {
                unimplemented!()
            }
        }
    }
}

impl Expression {
    pub fn position(&self) -> OffsetRange {
        match self {
            Expression::Data(v) => v.position(),
            Expression::UnarySuffix(v) => v.position,
            Expression::UnaryPrefix(v) => v.position,
            Expression::Concat(v) => v.position,
            Expression::Choice(v) => v.position,
            Expression::Mark(v) => v.position,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConcatExpressionResolver {
    pub base: Expression,
    pub rest: Vec<ConcatExpressionRest>,
}

#[derive(Clone, Debug)]
pub struct ConcatExpressionRest {
    pub expr: Expression,
    pub position: OffsetRange,
}

impl ConcatExpressionResolver {
    pub fn left_associative(self) -> ConcatExpression {

        let mut rest = self.rest.into_iter();
        let rhs = rest.next().unwrap();
        let position = join_position::<_, Rule>(self.base.position(), rhs.position);
        let mut out = ConcatExpression { lhs: self.base, rhs: rhs.expr, position };
        for term in rest {
            let position = join_position::<_, Rule>(out.position, rhs.position);
            out = ConcatExpression {
                lhs: Expression::Concat(Box::new(out)),
                rhs: term.expr,
                position,
            }
        }
        return out;
    }
}
