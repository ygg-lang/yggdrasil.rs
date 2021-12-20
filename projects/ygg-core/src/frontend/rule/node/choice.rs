use super::*;


#[derive(Clone, Eq, PartialEq)]
pub struct ChoiceExpression {
    pub inner: IndexSet<Expression>,
}

impl Expression {
    #[inline]
    pub fn choice(lhs: Term, rhs: Term) -> Self {
        let mut base = Expression::from(lhs).as_choice();
        base.get_choice_mut().map(|e| e.add_assign(Expression::from(rhs)));
        return base;
    }
    fn as_choice(self) -> Self {
        if let Some(_) = self.get_choice() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: Term::concat(self) };
    }
}

impl Term {
    pub fn choice(base: Expression) -> Self {
        let mut inner = IndexSet::default();
        inner.insert(base);
        Self::Choice(Box::new(ChoiceExpression { inner }))
    }
}

impl From<Term> for ChoiceExpression {
    fn from(e: Term) -> Self {
        Self::from(Expression::from(e))
    }
}

impl From<Expression> for ChoiceExpression {
    fn from(e: Expression) -> Self {
        match e.get_choice() {
            Some(s) => Self { inner: s.inner },
            None => {
                let mut inner = IndexSet::new();
                inner.insert(e);
                Self { inner }
            }
        }
    }
}

impl AddAssign<Expression> for ChoiceExpression {
    fn add_assign(&mut self, rhs: Expression) {
        match rhs.get_choice() {
            Some(c) => c.inner.into_iter().for_each(|e| {
                self.inner.insert(e);
            }),
            None => {
                self.inner.insert(rhs);
            }
        };
    }
}
