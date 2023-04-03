use super::*;

impl<'i> Extractor<Define_classContext<'i>> for GrammarRule {
    fn take_one(node: &Define_classContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.name.clone())?;
        let modifiers = YggdrasilModifiers::take(node.modifiers()).unwrap_or_default();
        let anno = YggdrasilAnnotations { macros: vec![], modifiers };
        let expr = YggdrasilExpression::take(node.class_block());
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(GrammarRule::create_class(id, expr, range).with_annotation(&anno))
    }
}

impl<'i> Extractor<Class_blockContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Class_blockContextAll<'i>) -> Option<Self> {
        let terms = YggdrasilExpression::take_many(&node.class_expression_all());
        Some(ConcatExpression::new(terms)?.into())
    }
}

impl<'i> Extractor<Class_expressionContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Class_expressionContextAll<'i>) -> Option<Self> {
        match node {
            Class_expressionContextAll::CSuffixContext(c) => {
                let suffix = YggdrasilOperator::take(c.suffix())?;
                let base = Self::take(c.class_expression())?;
                Some(YggdrasilExpression::unary(base, suffix))
            }
            Class_expressionContextAll::CETagContext(c) => {
                let name = YggdrasilIdentifier::take(c.identifier())?;
                let rule = YggdrasilExpression::take(c.class_expression())?;
                Some(rule.with_tag(name))
            }
            Class_expressionContextAll::CUntagContext(c) => {
                let base = YggdrasilExpression::take(c.class_expression())?;
                Some(base.with_remark())
            }
            Class_expressionContextAll::CSoftContext(c) => {
                let lhs = YggdrasilExpression::take(c.lhs.clone())?;
                let rhs = YggdrasilExpression::take(c.rhs.clone())?;
                Some(lhs & rhs)
            }
            Class_expressionContextAll::CHardContext(c) => {
                let lhs = YggdrasilExpression::take(c.lhs.clone())?;
                let rhs = YggdrasilExpression::take(c.rhs.clone())?;
                Some(lhs + rhs)
            }
            Class_expressionContextAll::CPatternContext(c) => {
                let lhs = YggdrasilExpression::take(c.lhs.clone())?;
                let rhs = YggdrasilExpression::take(c.rhs.clone())?;
                Some(lhs | rhs)
            }
            Class_expressionContextAll::AtomContext(c) => YggdrasilExpression::take(c.atomic()),
            Class_expressionContextAll::CNotContext(c) => {
                let base = Self::take(c.class_expression())?;
                Some(Self::unary(base, YggdrasilOperator::Negative))
            }
            Class_expressionContextAll::Error(_) => None,
        }
    }
}

impl<'i> Extractor<SuffixContextAll<'i>> for YggdrasilOperator {
    fn take_one(node: &SuffixContextAll<'i>) -> Option<Self> {
        match node {
            SuffixContextAll::OptionalContext(_) => Some(YggdrasilOperator::Optional),
            SuffixContextAll::MaybeContext(_) => Some(YggdrasilOperator::Optional),
            SuffixContextAll::MaybeGreedyContext(_) => Some(YggdrasilOperator::Repeats),
            SuffixContextAll::ManyGreedyContext(_) => Some(YggdrasilOperator::Repeat1),
            SuffixContextAll::ManyContext(_) => Some(YggdrasilOperator::Repeat1),
            SuffixContextAll::Error(_) => None,
        }
    }
}

// impl<'i> Extractor<NamepathContextAll<'i>> for RuleReference {
//     fn take_one(node: &NamepathContextAll<'i>) -> Option<Self> {
//         // node.identifier_all()
//     }
// }
