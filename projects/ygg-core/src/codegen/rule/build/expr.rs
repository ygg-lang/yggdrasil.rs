use super::*;

impl From<YGGRule> for Variable {
    fn from(rule: YGGRule) -> Self {
        Self { name: rule.name.data.to_owned(), kind: VariableType::Named, rule: Rule::from(rule.expression) }
    }
}

impl From<ExpressionNode> for Rule {
    fn from(e: ExpressionNode) -> Self {
        if !e.has_meta() {
            return match e.node {
                RefinedExpression::Data(e) => Self::from(*e),
                RefinedExpression::Unary(e) => Self::from(*e),
                RefinedExpression::Choice(e) => Self::from(*e),
                RefinedExpression::Concat(e) => Self::from(*e),
            };
        }
        unimplemented!()
    }
}

impl From<RefinedUnary> for Rule {
    fn from(unary: RefinedUnary) -> Self {
        let mut base = Rule::from(unary.base);
        for op in unary.ops {
            match op {
                Operator::Optional => base = Rule::Repeat(box base),
                Operator::Repeats => base = Rule::Repeat(box base),
                Operator::Repeats1 => base = Rule::Repeat(box base),
                Operator::Recursive | Operator::Mark => {
                    // unreachable!()
                }
            }
        }
        return base;
    }
}

impl From<RefinedConcat> for Rule {
    fn from(e: RefinedConcat) -> Self {
        Self::Choice(e.inner.iter().cloned().map(|e| e.into()).collect())
    }
}

impl From<RefinedData> for Rule {
    fn from(data: RefinedData) -> Self {
        match data {
            RefinedData::String(s) => Self::String(s),
            RefinedData::Integer(s) => Self::String(s.to_string()),
            RefinedData::Identifier(s) => Self::NamedSymbol(s.data),
            _ => unimplemented!()
        }
    }
}

impl From<RefinedChoice> for Rule {
    fn from(e: RefinedChoice) -> Self {
        Rule::Choice(e.inner.into_iter().map(|e| Rule::from(e)).collect())
    }
}
