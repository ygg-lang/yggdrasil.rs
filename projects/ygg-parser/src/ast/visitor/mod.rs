use super::*;
use yggdrasil_ir::{
    nodes::ExpressionNode,
    rule::{GrammarRule, GrammarRuleKind},
};

// mod atomic;
// mod looping;
//
// mod calls;
// mod collection;
// mod let_binding;
// mod modifiers;

impl ParseTreeVisitorCompat<'_> for YggdrasilParser {
    type Node = YggdrasilAntlrParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        unreachable!()
    }
    fn visit(&mut self, node: &<Self::Node as ParserNodeType<'_>>::Type) -> Self::Return {
        node.accept_dyn(self);
    }
}

/// Convert weakly typed ast to strongly typed ast
impl YggdrasilAntlrVisitor<'_> for YggdrasilParser {
    fn visit_program(&mut self, ctx: &ProgramContext<'_>) {
        for node in ctx.get_children() {
            if let Some(s) = node.downcast_ref::<Define_classContextAll>().and_then(GrammarRule::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Define_unionContextAll>().and_then(GrammarRule::take_one) {
                self.statements.push(s.into());
                continue;
            }
        }
    }
    fn visit_class_block(&mut self, ctx: &Class_blockContext<'_>) {
        todo!()
    }
}

impl<'i> Extractor<Define_classContextAll<'i>> for GrammarRule {
    fn take_one(node: &Define_classContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(Self {
            name: "".to_string(),
            kind: GrammarRuleKind::Class,
            r#type: "".to_string(),
            document: "".to_string(),
            public: false,
            derives: Default::default(),
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            body: ExpressionNode { kind: (), tag: "".to_string() },
            range: span,
        })
    }
}

impl<'i> Extractor<Define_unionContextAll<'i>> for GrammarRule {
    fn take_one(node: &Define_unionContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(Self {
            name: "".to_string(),
            kind: GrammarRuleKind::Union,
            r#type: "".to_string(),
            document: "".to_string(),
            public: false,
            derives: Default::default(),
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            body: ExpressionNode { kind: (), tag: "".to_string() },
            range: span,
        })
    }
}
