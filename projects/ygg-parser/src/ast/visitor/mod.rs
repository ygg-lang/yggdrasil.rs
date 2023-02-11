use super::*;
use antlr_rust::tree::{ErrorNode, VisitChildren};
use yggdrasil_ir::{
    data::DataKind,
    nodes::{ExpressionKind, ExpressionNode},
    rule::{GrammarRule, GrammarRuleKind, YggdrasilIdentifier},
};

mod atomic;
mod classes;
// mod calls;
// mod collection;
// mod let_binding;
// mod modifiers;

impl ParseTreeVisitorCompat<'_> for YggdrasilParser {
    type Node = YggdrasilAntlrParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.dirty
    }
}

/// Convert weakly typed ast to strongly typed ast
impl YggdrasilAntlrVisitor<'_> for YggdrasilParser {
    fn visit_define_class(&mut self, ctx: &Define_classContext<'_>) {
        if let Some(s) = GrammarRule::take_one(ctx) {
            self.grammar.insert(s);
        }
    }
    fn visit_define_union(&mut self, ctx: &Define_unionContext<'_>) {
        if let Some(s) = GrammarRule::take_one(ctx) {
            self.grammar.insert(s);
        }
    }
}

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
