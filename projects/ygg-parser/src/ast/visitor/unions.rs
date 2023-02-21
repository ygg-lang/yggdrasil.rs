use super::*;
use yggdrasil_ir::rule::GrammarAtomic;

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
        Some(YggdrasilExpression::any())
    }
}
