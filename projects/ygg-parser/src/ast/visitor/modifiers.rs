use super::*;

use yggdrasil_ir::rule::{YggdrasilMacroCall, YggdrasilModifiers};

impl<'i> Extractor<ModifiersContextAll<'i>> for YggdrasilModifiers {
    fn take_one(node: &ModifiersContextAll<'i>) -> Option<Self> {
        let modifiers = YggdrasilIdentifier::take_many(&node.identifier_all());
        Some(Self { identifiers: modifiers })
    }
}

impl<'i> Extractor<Macro_callContextAll<'i>> for YggdrasilMacroCall {
    fn take_one(node: &Macro_callContextAll<'i>) -> Option<Self> {
        let name = YggdrasilNamepath::take(node.namepath())?;
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        let tuple = node.tuple_block()
        Some(Self { name, range })
    }
}

pub struct Args {
    args: Vec<(  YggdrasilExpression)>,
}

impl<'i> Extractor<Tuple_blockContextAll<'i>> for Args {
    fn take_one(node: &Tuple_blockContextAll<'i>) -> Option<Self> {
        YggdrasilExpression::take_many(node.class_expression())
    }
}