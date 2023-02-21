use super::*;
use yggdrasil_ir::{
    nodes::{ChoiceExpression, ConcatExpression},
    rule::GrammarAtomic,
};

impl<'i> Extractor<Define_unionContext<'i>> for GrammarRule {
    fn take_one(node: &Define_unionContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.name.clone())?;
        let expr = YggdrasilExpression::take(node.union_block());
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(GrammarRule::create_union(id, range).with_expression(expr))
    }
}

impl<'i> Extractor<Union_blockContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Union_blockContextAll<'i>) -> Option<Self> {
        let terms = YggdrasilExpression::take_many(&node.union_term_all());
        if terms.len() == 1 { return terms.first().cloned() } else { Some(ChoiceExpression { branches: terms }.into()) }
    }
}

impl<'i> Extractor<Union_termContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Union_termContextAll<'i>) -> Option<Self> {
        let terms = YggdrasilExpression::take_many(&node.union_expression_all());
        Some(ConcatExpression { sequence: terms }.into())
    }
}

impl<'i> Extractor<Union_expressionContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Union_expressionContextAll<'i>) -> Option<Self> {
        match node {
            Union_expressionContextAll::UHardContext(v) => {
                let lhs = YggdrasilExpression::take(v.lhs.clone())?;
                let rhs = YggdrasilExpression::take(v.rhs.clone())?;
                Some(lhs + rhs)
            }
            Union_expressionContextAll::USoftContext(v) => {
                let lhs = YggdrasilExpression::take(v.lhs.clone())?;
                let rhs = YggdrasilExpression::take(v.rhs.clone())?;
                Some(lhs & rhs)
            }
            Union_expressionContextAll::UUntagContext(_) => {
                todo!()
            }
            Union_expressionContextAll::USuffixContext(_) => {
                todo!()
            }
            Union_expressionContextAll::UGroupContext(_) => {
                todo!()
            }
            Union_expressionContextAll::UETagContext(_) => {
                todo!()
            }
            Union_expressionContextAll::UtomContext(v) => YggdrasilExpression::take(v.atomic()),
            Union_expressionContextAll::UNotContext(_) => {
                todo!()
            }

            Union_expressionContextAll::UCallContext(_) => {
                todo!()
            }
            Union_expressionContextAll::Error(_) => None,
        }
    }
}
