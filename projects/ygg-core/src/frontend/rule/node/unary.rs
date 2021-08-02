use super::*;

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
    pub fn prefix(e: Expression, o: &str) -> Self {
        let mut node = Self::from(e).as_unary();
        node.get_unary_mut().map(|e| e.ops.push(Operator::prefix(o)));
        return node;
    }
    pub fn suffix(e: Expression, o: &str) -> Self {
        let mut node = Self::from(e).as_unary();
        node.get_unary_mut().map(|e| e.ops.push(Operator::suffix(o)));
        return node;
    }
    fn as_unary(self) -> Self {
        if let Some(_) = self.get_unary() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: RefinedExpression::unary(self) };
    }
    pub fn mark_node(name: Expression, node: Expression) -> Self {
        let mut node = Self::from(node);
        node.node_tag = name.as_symbol();
        return node;
    }
    pub fn mark_branch(base: Expression, kind: Option<char>, name: Symbol) -> Self {
        let mut node = Self::from(base);
        node.branch_tag = Some(ExpressionTag::new(kind, name));
        return node;
    }

}

impl ExpressionTag {
    pub fn new(kind: Option<char>, name: Symbol) -> Self {
        match kind {
            None => {
                unimplemented!()
            }
            Some(_) => {
                unimplemented!()
            }
        }
        Self { name, mode: "".to_string() }
    }
}

impl RefinedExpression {
    pub fn unary(base: ExpressionNode) -> Self {
        Self::Unary(Box::new(RefinedUnary { base, ops: vec![] }))
    }
}