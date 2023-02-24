use super::*;

use yggdrasil_ir::{
    data::{YggdrasilRegex, YggdrasilText},
    nodes::{Operator, YggdrasilExpression},
    rule::{GrammarRule, YggdrasilAnnotations, YggdrasilIdentifier, YggdrasilNamepath},
};

mod atomic;
mod classes;
mod unions;
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
    fn visit_define_grammar(&mut self, ctx: &Define_grammarContext<'_>) {
        self.grammar.name = YggdrasilIdentifier::take(ctx.identifier()).unwrap();
    }
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
