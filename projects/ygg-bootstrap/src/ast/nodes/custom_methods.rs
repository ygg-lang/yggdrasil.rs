use super::*;

impl Expression {
    pub fn range(&self) -> (usize, usize) {
        match self {
            Expression::Data(e) => e.range(),
            Expression::Concat { lhs, rhs, .. } => (lhs.range().0, rhs.range().1),
            Expression::Choice { lhs, rhs } => (lhs.range().0, rhs.range().1),
            Expression::MarkNode { lhs, rhs } => (lhs.range().0, rhs.range().1),
            Expression::MarkNodeShort(e) => (e.range().0 - 1, e.range().1),
            Expression::MarkType { lhs, rhs } => (lhs.range().0, rhs.range().1),
            Expression::MarkBranch { base, name, .. } => (base.range().0, name.range.1),
            Expression::MustNot(e) => (e.range().0 - 1, e.range().1),
            Expression::MustOne(e) => (e.range().0 - 1, e.range().1),
            Expression::Maybe(e) => (e.range().0, e.range().1 + 1),
            Expression::Many(e) => (e.range().0, e.range().1 + 1),
            Expression::ManyNonNull(e) => (e.range().0, e.range().1 + 1),
        }
    }
}

impl Data {
    pub fn range(&self) -> (usize, usize) {
        match self {
            Data::Symbol(e) => e.range,
            Data::Integer(e) => e.range,
            Data::String(e) => e.range,
            Data::Macro => (0, 0),
            Data::Regex => (0, 0),
        }
    }
}
