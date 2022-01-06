use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ConcatExpression {
    pub sequence: Vec<ExpressionNode>,
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
