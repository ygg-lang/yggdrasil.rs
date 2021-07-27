use super::*;

impl RefinedUnary {
    pub fn build_prefix(e: ExpressionNode, o: String) -> Self {
        let mut base = Self::from(e);
        base.ops.push(Operator::prefix(&o));
        return base;
    }
    pub fn build_suffix(e: ExpressionNode, o: String) -> Self {
        let mut base = Self::from(e);
        base.ops.push(Operator::suffix(&o));
        return base;
    }
}

impl Operator {
    pub fn prefix(o: &str) -> Operator {
        match o {
            "*" => Self::Recursive,
            "^" => Self::Mark,
            _ => unreachable!(),
        }
    }
    pub fn suffix(o: &str) -> Operator {
        match o {
            "?" => Self::Optional,
            "+" => Self::Repeats,
            "*" => Self::Repeats1,
            _ => unreachable!(),
        }
    }
}

impl From<ExpressionNode> for RefinedUnary {
    fn from(e: ExpressionNode) -> Self {
        match e.get_unary() {
            Some(s) => s,
            None => Self { base: e, ops: vec![] },
        }
    }
}
