use super::*;
use yggdrasil_ir::nodes::Operator;

impl<'i> Extractor<Define_classContext<'i>> for GrammarRule {
    fn take_one(node: &Define_classContext<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.name.clone())?;
        let expr = ExpressionNode::take(node.class_block())?;
        Some(GrammarRule {
            name: id,
            kind: GrammarRuleKind::Class,
            r#type: "".to_string(),
            document: "".to_string(),
            public: false,
            derives: Default::default(),
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            body: expr,
            range: Default::default(),
        })
    }
}
impl<'i> Extractor<Class_blockContextAll<'i>> for ExpressionNode {
    fn take_one(node: &Class_blockContextAll<'i>) -> Option<Self> {
        let terms = ExpressionNode::take_many(&node.class_expression_all());
        println!("{:?}", terms);
        None
    }
}

impl<'i> Extractor<Class_expressionContextAll<'i>> for ExpressionNode {
    fn take_one(node: &Class_expressionContextAll<'i>) -> Option<Self> {
        match node {
            Class_expressionContextAll::CSuffixContext(s) => {
                let suffix = Operator::take(s.suffix())?;
                let base = Self::take(s.class_expression())?;
                Some(ExpressionNode::unary(base, suffix))
            }
            Class_expressionContextAll::CCallContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CETagContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CUntagContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CSoftContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CHardContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CPatternContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CGroupContext(_) => {
                todo!()
            }
            Class_expressionContextAll::AtomContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CNotContext(_) => {
                todo!()
            }
            Class_expressionContextAll::Error(_) => None,
        }
    }
}
impl<'i> Extractor<SuffixContextAll<'i>> for Operator {
    fn take_one(node: &SuffixContextAll<'i>) -> Option<Self> {
        match node {
            SuffixContextAll::OptionalContext(_) => Some(Operator::Optional),
            SuffixContextAll::MaybeContext(_) => Some(Operator::Optional),
            SuffixContextAll::MaybeGreedyContext(_) => Some(Operator::Optional),
            SuffixContextAll::ManyGreedyContext(_) => Some(Operator::Optional),
            SuffixContextAll::ManyContext(_) => Some(Operator::Optional),
            SuffixContextAll::Error(_) => None,
        }
    }
}
