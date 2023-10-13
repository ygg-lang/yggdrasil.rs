use super::*;

impl<'i> Extractor<ModifiersContextAll<'i>> for YggdrasilModifiers {
    fn take_one(node: &ModifiersContextAll<'i>) -> Option<Self> {
        let modifiers = YggdrasilIdentifier::take_many(&node.identifier_all());
        Some(Self { identifiers: modifiers })
    }
}
impl<'i> Extractor<AnnotationContextAll<'i>> for YggdrasilMacroCall {
    fn take_one(node: &AnnotationContextAll<'i>) -> Option<Self> {
        let name = YggdrasilNamepath::take(node.namepath())?;
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        let arguments = match node.tuple_block() {
            Some(s) => YggdrasilMacroArgument::take_many(&s.class_expression_all()),
            None => vec![],
        };
        Some(Self { name, arguments, range })
    }
}

impl<'i> Extractor<Macro_callContextAll<'i>> for YggdrasilMacroCall {
    fn take_one(node: &Macro_callContextAll<'i>) -> Option<Self> {
        let name = YggdrasilNamepath::take(node.namepath())?;
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        let arguments = match node.tuple_block() {
            Some(s) => YggdrasilMacroArgument::take_many(&s.class_expression_all()),
            None => vec![],
        };
        Some(Self { name, arguments, range })
    }
}

impl<'i> Extractor<Class_expressionContextAll<'i>> for YggdrasilMacroArgument {
    fn take_one(node: &Class_expressionContextAll<'i>) -> Option<Self> {
        let mut value = YggdrasilExpression::take_one(node)?;
        let key = take(&mut value.tag);
        Some(Self { key, value })
    }
}
