use super::*;

impl<'i> Extractor<Token_pairContextAll<'i>> for GrammarRule {
    fn take_one(node: &Token_pairContextAll<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.identifier().clone())?;
        let modifiers = YggdrasilModifiers::take(node.modifiers()).unwrap_or_default();
        let macros = YggdrasilMacroCall::take_many(&node.annotation_all());
        let anno = YggdrasilAnnotations { macros, modifiers };
        let expr = YggdrasilExpression::take(node.atomic());
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(GrammarRule::create_class(id, expr, range).with_annotation(&anno))
    }
}
