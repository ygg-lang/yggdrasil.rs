use super::*;


impl From<UnaryPrefix> for RefinedUnary {
    fn from(e: UnaryPrefix) -> Self {
        Self {
            base: ExpressionNode::from(e.base),
            prefix: vec![e.prefix],
            suffix: vec![]
        }
    }
}

impl From<UnarySuffix> for RefinedUnary {
    fn from(e: UnarySuffix) -> Self {
        Self {
            base: ExpressionNode::from(e.base),
            prefix: vec![],
            suffix: vec![e.suffix]
        }
    }
}