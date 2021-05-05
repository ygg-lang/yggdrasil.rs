use super::*;

impl From<ConcatExpression> for RefinedConcat {
    fn from(e: ConcatExpression) -> Self {
        let lhs = RefinedExpression::from(e.lhs);
        let rhs = RefinedExpression::from(e.rhs);
        let mut base = Self::from(lhs);
        base += rhs;
        return base;
    }
}

impl From<RefinedExpression> for RefinedConcat {
    fn from(e: RefinedExpression) -> Self {
        match e {
            RefinedExpression::Concat(c) => Self { inner: c.inner },
            _ => Self { inner: vec![e] },
        }
    }
}

impl AddAssign<RefinedExpression> for RefinedConcat {
    fn add_assign(&mut self, rhs: RefinedExpression) {
        match rhs {
            RefinedExpression::Concat(c) => self.inner.extend(c.inner),
            _ => self.inner.push(rhs),
        }
    }
}