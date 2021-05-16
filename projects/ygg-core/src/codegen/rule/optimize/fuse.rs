use super::*;

impl YGGRule {
    pub(super) fn fuse(&mut self) {
        self.expression.fuse()
    }
}

impl ExpressionNode {
    fn fuse(&mut self) {
        self.swap_empty();
        match &mut self.node {
            RefinedExpression::Data(_) => {}
            RefinedExpression::Unary(e) => {e.fuse()}
            RefinedExpression::Choice(e) => {e.fuse()}
            RefinedExpression::Concat(e) => {e.fuse()}
        }
    }
    fn swap_empty(&mut self) {
        match &self.node {
            RefinedExpression::Data(_) => (),
            RefinedExpression::Unary(e) => {
                // ?
                if e.ops.is_empty()&&self.meta_eq(&e.base) {
                    *self = e.base.to_owned()
                }
            }
            RefinedExpression::Choice(e) => {
                // (x <- a) | a
                if e.inner.len() == 1 && self.meta_eq(&e.inner[0]){
                    *self = e.inner[0].to_owned()
                }
            }
            RefinedExpression::Concat(e) => {
                // a <- ( b <- c)
                if e.inner.len() == 1 && self.meta_eq(&e.inner[0]){
                    *self = e.inner[0].to_owned()
                }
            }
        }
    }

    fn meta_eq(&self, rhs: &Self) -> bool {
        rhs.inline_token == self.inline_token && rhs.field == self.field && rhs.tag == self.tag && rhs.ty == self.ty
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
