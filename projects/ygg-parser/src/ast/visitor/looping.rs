use super::*;

impl<'i> Extractor<Loop_statementContextAll<'i>> for StatementNode {
    fn take_one(node: &Loop_statementContextAll<'i>) -> Option<Self> {
        let body: StatementNode = match node {
            Loop_statementContextAll::ForLoopContext(s) => ForLoop::take_one(s)?.into(),
            Loop_statementContextAll::WhileLetContext(s) => WhileLoop::take_one(s)?.into(),
            Loop_statementContextAll::WhileLoopContext(s) => WhileLoop::take_one(s)?.into(),
            Loop_statementContextAll::Error(_) => None?,
        };
        Some(body)
    }
}

impl<'i> Extractor<ForLoopContext<'i>> for ForLoop {
    fn take_one(node: &ForLoopContext<'i>) -> Option<Self> {
        let iter = ExpressionType::take(node.cond.clone())?;
        let guard = ExpressionType::take(node.guard.clone());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self {
            pattern: LetPattern::Tuple(Box::new(TuplePatternNode {
                bind: None,
                name: None,
                terms: vec![],
                span: span.clone(),
            })),
            iterator: iter,
            condition: guard,
            then_body: Default::default(),
            span,
        })
    }
}
impl<'i> Extractor<WhileLoopContext<'i>> for WhileLoop {
    fn take_one(node: &WhileLoopContext<'i>) -> Option<Self> {
        let iter = ExpressionType::take(node.cond.clone())?;
        // let guard = ExpressionType::take(node.guard.clone());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self {
            kind: WhileLoopKind::While,
            condition: WhileConditionNode::Unconditional,
            then_body: Default::default(),
            otherwise: None,
            span,
        })
    }
}

impl<'i> Extractor<WhileLetContext<'i>> for WhileLoop {
    fn take_one(node: &WhileLetContext<'i>) -> Option<Self> {
        // let iter = ExpressionType::take(node.cond.clone())?;
        // let guard = ExpressionType::take(node.guard.clone());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self {
            kind: WhileLoopKind::While,
            condition: WhileConditionNode::Unconditional,
            then_body: Default::default(),
            otherwise: None,
            span,
        })
    }
}
