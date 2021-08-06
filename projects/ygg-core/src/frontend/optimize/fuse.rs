use super::*;

impl Rule {
    pub(crate) fn fuse(&mut self) {
        self.expression.fuse()
    }
}

impl ExpressionNode {
    fn fuse(&mut self) {
        self.swap_empty();
        match &mut self.node {
            RefinedExpression::Data(_) => {}
            RefinedExpression::Unary(e) => e.fuse(),
            RefinedExpression::Choice(e) => e.fuse(),
            RefinedExpression::Concat(e) => e.fuse(),
        }
    }
    fn swap_empty(&mut self) {
        match &self.node {
            RefinedExpression::Data(_) => (),
            RefinedExpression::Unary(e) => {
                // empty a
                if e.ops.is_empty() && self.meta_eq(&e.base) {
                    *self = e.base.to_owned()
                }
            }
            RefinedExpression::Choice(e) => {
                // a | a
                if e.inner.len() == 1 {
                    let item = e.inner.iter().next().unwrap();
                    if self.meta_eq(item) {
                        *self = item.to_owned()
                    }
                }
            }
            RefinedExpression::Concat(e) => {
                // empty a
                if e.rest.is_empty() && self.meta_eq(&e.base) {
                    *self = e.base.to_owned()
                }
            }
        }
    }

    fn meta_eq(&self, rhs: &Self) -> bool {
        rhs.inline_token == self.inline_token
            && rhs.node_tag == self.node_tag
            && rhs.branch_tag == self.branch_tag
            && rhs.ty == self.ty
    }
}

impl RefinedUnary {
    fn fuse(&mut self) {}
}

impl RefinedChoice {
    fn fuse(&mut self) {}
}

impl RefinedConcat {
    fn fuse(&mut self) {}
}
