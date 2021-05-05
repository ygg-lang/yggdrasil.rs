use super::*;


impl From<ChoiceExpression> for RefinedChoice {
    fn from(e: ChoiceExpression) -> Self {
        let lhs = RefinedExpression::from(e.lhs);
        let rhs = RefinedExpression::from(e.rhs);
        let mut base = Self::from(lhs);
        base += rhs;
        return base;
    }
}

impl From<ChoiceTag> for RefinedExpression {
    fn from(e: ChoiceTag) -> Self {
        RefinedExpression::Choice(box RefinedChoice {
            inner: vec![RefinedTag::from(e)]
        })
    }
}

impl From<RefinedExpression> for RefinedChoice {
    fn from(_: RefinedExpression) -> Self {
        todo!()
    }
}

impl From<ChoiceTag> for RefinedTag {
    fn from(e: ChoiceTag) -> Self {
        Self { expr: e.expr.into() }
    }
}

impl AddAssign<RefinedExpression> for RefinedChoice {
    fn add_assign(&mut self, rhs: RefinedExpression) {
        match rhs {
            RefinedExpression::Choice(c) => self.inner.extend(c.inner),
            _ => self.inner.push(rhs),
        }
    }
}