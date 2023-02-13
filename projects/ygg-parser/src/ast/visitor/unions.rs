use super::*;

impl<'i> Extractor<Define_unionContext<'i>> for GrammarRule {
    fn take_one(node: &Define_unionContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.name.clone())?;
        let expr = ExpressionNode::take(node.union_block())?;
        Some(GrammarRule {
            name: id,
            kind: GrammarRuleKind::Union,
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
impl<'i> Extractor<Union_blockContextAll<'i>> for ExpressionNode {
    fn take_one(node: &Union_blockContextAll<'i>) -> Option<Self> {
        None
    }
}
