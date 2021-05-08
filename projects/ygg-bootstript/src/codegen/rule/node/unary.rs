use super::*;

impl From<UnaryPrefix> for ExpressionNode {
    fn from(e: UnaryPrefix) -> Self {
        Self { tag: None, ty: None, field: None, node: RefinedExpression::Unary(box RefinedUnary::from(e)) }
    }
}

impl From<UnaryPrefix> for RefinedUnary {
    fn from(e: UnaryPrefix) -> Self {
        Self { base: ExpressionNode::from(e.base), prefix: vec![e.prefix], suffix: vec![] }
    }
}

impl From<UnarySuffix> for ExpressionNode {
    fn from(e: UnarySuffix) -> Self {
        Self { tag: None, ty: None, field: None, node: RefinedExpression::Unary(box RefinedUnary::from(e)) }
    }
}

impl From<UnarySuffix> for RefinedUnary {
    fn from(e: UnarySuffix) -> Self {
        Self { base: ExpressionNode::from(e.base), prefix: vec![], suffix: vec![e.suffix] }
    }
}
