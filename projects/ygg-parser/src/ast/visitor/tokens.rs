use super::*;

pub struct BatchClass {
    pub id: Option<YggdrasilIdentifier>,
    pub classes: Vec<GrammarRule>,
}

struct LazyClass {
    class: GrammarRule,
    inner: YggdrasilAnnotations,
}

impl<'i> Extractor<Define_tokenContext<'i>> for BatchClass {
    fn take_one(node: &Define_tokenContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.identifier().clone());
        let modifiers = YggdrasilModifiers::take(node.modifiers()).unwrap_or_default();
        let macros = YggdrasilMacroCall::take_many(&node.annotation_all());
        let outer = YggdrasilAnnotations { auto_tag: false, macros, modifiers };
        let classes = LazyClass::take_many(&node.token_block()?.token_pair_all())
            .into_iter()
            .map(|LazyClass { class, inner: annotation }| class.with_annotation(&outer).with_annotation(&annotation))
            .collect();
        Some(BatchClass { id, classes })
    }
}

impl<'i> Extractor<Token_pairContextAll<'i>> for LazyClass {
    fn take_one(node: &Token_pairContextAll<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.identifier().clone())?;
        let modifiers = YggdrasilModifiers::take(node.modifiers()).unwrap_or_default();
        let macros = YggdrasilMacroCall::take_many(&node.annotation_all());
        let annotation = YggdrasilAnnotations { auto_tag: false, macros, modifiers };
        let expr = YggdrasilExpression::take(node.atomic());
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(LazyClass { class: GrammarRule::create_class(id, expr, range), inner: annotation })
    }
}
