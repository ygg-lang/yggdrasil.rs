use super::*;
use yggdrasil_ir::nodes::ConcatExpression;

impl<'i> Extractor<Define_classContext<'i>> for GrammarRule {
    fn take_one(node: &Define_classContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.name.clone())?;
        let modifiers = YggdrasilIdentifier::take_many(&node.mods);
        let anno = YggdrasilAnnotations { macros: vec![], modifiers };
        let expr = YggdrasilExpression::take(node.class_block());
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(GrammarRule::create_class(id, range).with_annotation(&anno).with_expression(expr))
    }
}

impl<'i> Extractor<Class_blockContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Class_blockContextAll<'i>) -> Option<Self> {
        let terms = YggdrasilExpression::take_many(&node.class_expression_all());
        Some(ConcatExpression { sequence: terms }.into())
    }
}

impl<'i> Extractor<Class_expressionContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Class_expressionContextAll<'i>) -> Option<Self> {
        match node {
            Class_expressionContextAll::CSuffixContext(s) => {
                let suffix = Operator::take(s.suffix())?;
                let base = Self::take(s.class_expression())?;
                Some(YggdrasilExpression::unary(base, suffix))
            }
            Class_expressionContextAll::CCallContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CETagContext(e) => {
                let name = YggdrasilIdentifier::take(e.identifier())?;
                let rule = YggdrasilExpression::take(e.class_expression())?;
                Some(rule.with_tag(name))
            }
            Class_expressionContextAll::CUntagContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CSoftContext(v) => {
                let lhs = YggdrasilExpression::take(v.lhs.clone())?;
                let rhs = YggdrasilExpression::take(v.rhs.clone())?;
                Some(lhs & rhs)
            }
            Class_expressionContextAll::CHardContext(v) => {
                let lhs = YggdrasilExpression::take(v.lhs.clone())?;
                let rhs = YggdrasilExpression::take(v.rhs.clone())?;
                Some(lhs + rhs)
            }
            Class_expressionContextAll::CPatternContext(v) => {
                let lhs = YggdrasilExpression::take(v.lhs.clone())?;
                let rhs = YggdrasilExpression::take(v.rhs.clone())?;
                Some(lhs | rhs)
            }
            Class_expressionContextAll::CGroupContext(v) => YggdrasilExpression::take(v.class_expression()),
            Class_expressionContextAll::AtomContext(s) => YggdrasilExpression::take(s.atomic()),
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

// impl<'i> Extractor<NamepathContextAll<'i>> for RuleReference {
//     fn take_one(node: &NamepathContextAll<'i>) -> Option<Self> {
//         // node.identifier_all()
//     }
// }
