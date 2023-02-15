use super::*;
use std::ops::Add;
use yggdrasil_ir::{
    data::{RegularExpression, RuleReference, YggdrasilText},
    nodes::Operator,
};

impl<'i> Extractor<Define_classContext<'i>> for GrammarRule {
    fn take_one(node: &Define_classContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.name.clone())?;
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
            ignored: false,
            body: expr,
            range: Default::default(),
        })
    }
}
impl<'i> Extractor<Class_blockContextAll<'i>> for ExpressionNode {
    fn take_one(node: &Class_blockContextAll<'i>) -> Option<Self> {
        let terms = ExpressionNode::take_many(&node.class_expression_all());
        let expr = match terms.as_slice() {
            [head, rest @ ..] => {
                let mut out = head.clone();
                for item in rest {
                    out &= item.clone();
                }
                out
            }
            _ => ExpressionNode::empty(),
        };
        Some(expr)
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
            Class_expressionContextAll::CETagContext(e) => {
                let tag = e.tag_pair()?;
                let name = YggdrasilIdentifier::take(tag.lhs.clone())?;
                let rule = YggdrasilIdentifier::take(tag.rhs.clone())?;
                Some(RuleReference::new(name).with_tag(rule).into())
            }
            Class_expressionContextAll::CUntagContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CSoftContext(v) => {
                let lhs = ExpressionNode::take(v.lhs.clone())?;
                let rhs = ExpressionNode::take(v.rhs.clone())?;
                Some(lhs & rhs)
            }
            Class_expressionContextAll::CHardContext(v) => {
                let lhs = ExpressionNode::take(v.lhs.clone())?;
                let rhs = ExpressionNode::take(v.rhs.clone())?;
                Some(lhs + rhs)
            }
            Class_expressionContextAll::CPatternContext(_) => {
                todo!()
            }
            Class_expressionContextAll::CGroupContext(_) => {
                todo!()
            }
            Class_expressionContextAll::AtomContext(s) => ExpressionNode::take(s.atomic()),
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

impl<'i> Extractor<AtomicContextAll<'i>> for ExpressionNode {
    fn take_one(node: &AtomicContextAll<'i>) -> Option<Self> {
        match node {
            AtomicContextAll::AIntContext(_) => todo!(),
            AtomicContextAll::AReContext(r) => Some(RegularExpression::take(r.regex())?.into()),
            AtomicContextAll::ACharContext(_) => todo!(),
            AtomicContextAll::ATupleContext(_) => todo!(),
            AtomicContextAll::ASpecialContext(_) => todo!(),
            AtomicContextAll::AIdContext(s) => None,
            AtomicContextAll::AStringContext(s) => Some(YggdrasilText::take(s.string())?.into()),
            AtomicContextAll::Error(_) => None,
        }
    }
}

// impl<'i> Extractor<NamepathContextAll<'i>> for RuleReference {
//     fn take_one(node: &NamepathContextAll<'i>) -> Option<Self> {
//         // node.identifier_all()
//     }
// }
