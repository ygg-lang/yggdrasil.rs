use super::*;
use std::ops::{Add, BitAnd};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatExpression {
    pub sequence: Vec<ExpressionNode>,
}

impl ConcatExpression {
    pub fn new(lhs: impl Into<ExpressionNode>, rhs: impl Into<ExpressionNode>, soft: bool) -> Self {
        let mut sequence = vec![];
        sequence.push(lhs.into());
        if soft {}
        sequence.push(rhs.into());
        Self { sequence }
    }
}

impl Add<Self> for ExpressionNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let concat = ConcatExpression::new(self, rhs, true);
        ExpressionNode { kind: ExpressionKind::Concat(Box::new(concat)), tag: "".to_string() }
    }
}

impl BitAnd<Self> for ExpressionNode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let concat = ConcatExpression::new(self, rhs, false);
        ExpressionNode { kind: ExpressionKind::Concat(Box::new(concat)), tag: "".to_string() }
    }
}

// impl Expression {
//     #[inline]
//     pub fn concat(lhs: Term, rhs: Term) -> Self {
//         let mut base = Expression::from(lhs).as_concat();
//         base.get_concat_mut().map(|e| e.bitand_assign(Expression::from(rhs)));
//         return base;
//     }
//     #[inline]
//     pub fn soft_concat(lhs: Term, rhs: Term) -> Self {
//         let mut base = Expression::from(lhs).as_concat();
//         base.get_concat_mut().map(|e| e.add_assign(Expression::from(rhs)));
//         return base;
//     }
//     fn as_concat(self) -> Self {
//         if let Some(_) = self.get_concat() {
//             return self;
//         }
//         return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: Term::concat(self) };
//     }
// }
//
// impl Term {
//     pub fn concat(base: Expression) -> Self {
//         Self::Concat(Box::new(ConcatExpression { base, rest: vec![] }))
//     }
// }
//
// impl AddAssign<Expression> for ConcatExpression {
//     /// a + b
//     fn add_assign(&mut self, rhs: Expression) {
//         match rhs.get_concat() {
//             Some(c) => {
//                 self.rest.push((true, c.base));
//                 self.rest.extend(c.rest);
//             }
//             None => self.rest.push((true, rhs)),
//         }
//     }
// }
//
// impl BitAndAssign<Expression> for ConcatExpression {
//     /// a b
//     fn bitand_assign(&mut self, rhs: Expression) {
//         match rhs.get_concat() {
//             Some(c) => {
//                 self.rest.push((false, c.base));
//                 self.rest.extend(c.rest);
//             }
//             None => self.rest.push((false, rhs)),
//         }
//     }
// }
