use super::*;

impl RefinedUnary {
    pub fn build_prefix(e: ExpressionNode, o: &str) -> Self {
        let mut base = Self::from(e);
        base.ops.push(Operator::prefix(o));
        return base;
    }
    pub fn build_suffix(e: ExpressionNode, o: &str) -> Self {
        let mut base = Self::from(e);
        base.ops.push(Operator::suffix(o));
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

impl ExpressionNode {
    pub fn suffix(e: Expression, o: &str) -> Self {
        let mut node = Self::from(e);
        match node.get_unary_mut() {
            Some(s) => {s.ops.push(Operator::suffix(o))},
            None => {
                node = Self {
                    inline_token: false,
                    ty: None,
                    branch_tag: None,
                    node_tag: None,
                    node: RefinedExpression::Unary(Box::new(RefinedUnary::build_suffix(node, o)))
                }
            }
        }
        return node
    }
    pub fn mark_node(name: Expression, node: Expression) -> Self {
        let mut node = Self::from(node);
        node.node_tag = name.as_symbol();
        return node
    }
    pub fn mark_branch(base: Expression, kind: Option<char>, name:Symbol) -> Self {
        let mut node = Self::from(base);
        node.branch_tag = Some(ExpressionTag::new(kind, name));
        return node
    }
}

impl ExpressionTag {
    pub fn new(kind: Option<char>, name:Symbol) ->Self {
        match kind {
            None => {unimplemented!()}
            Some(_) => {unimplemented!()}
        }
        Self {
            name,
            mode: "".to_string()
        }
    }
}

impl From<ExpressionNode> for RefinedUnary {
    #[inline]
    fn from(e: ExpressionNode) -> Self {
        match e.get_unary() {
            Some(s) => s,
            None => Self { base: e, ops: vec![] },
        }
    }
}
