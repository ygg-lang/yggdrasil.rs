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

impl ASTNode {
    pub fn prefix(e: ASTExpression, o: &str) -> Self {
        let mut node = Self::from(e).as_unary();
        node.get_unary_mut().map(|e| e.ops.push(Operator::prefix(o)));
        return node;
    }
    pub fn suffix(e: ASTExpression, o: &str) -> Self {
        let mut node = Self::from(e).as_unary();
        node.get_unary_mut().map(|e| e.ops.push(Operator::suffix(o)));
        return node;
    }
    fn as_unary(self) -> Self {
        if let Some(_) = self.get_unary() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: ASTExpression::unary(self) };
    }
    pub fn mark_node(name: ASTExpression, node: ASTExpression) -> Self {
        let mut node = Self::from(node);
        node.node_tag = name.as_symbol();
        return node;
    }
    pub fn mark_branch(base: ASTExpression, kind: Option<char>, name: Symbol) -> Self {
        let mut node = Self::from(base);
        node.branch_tag = Some(ExpressionTag::new(kind, name));
        return node;
    }
}

impl ExpressionTag {
    pub fn new(kind: Option<char>, name: Symbol) -> Self {
        let mode = match kind {
            None => "".to_string(),
            Some(_) => {
                unimplemented!()
            }
        };
        Self { name, mode }
    }
}

impl ASTExpression {
    pub fn unary(base: ASTNode) -> Self {
        Self::Unary(Box::new(RefinedUnary { base, ops: vec![] }))
    }
}
