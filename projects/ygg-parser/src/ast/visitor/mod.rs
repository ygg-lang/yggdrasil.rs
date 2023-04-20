use super::*;

mod atomic;
mod classes;
mod unions;
// mod calls;
// mod collection;
// mod let_binding;
mod modifiers;

mod tokens;

use crate::ast::visitor::tokens::BatchClass;
use std::{mem::take, str::FromStr};
use yggdrasil_ir::{
    data::{YggdrasilRegex, YggdrasilText},
    nodes::{ConcatExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{
        BigInt, GrammarRule, YggdrasilAnnotations, YggdrasilIdentifier, YggdrasilMacroArgument, YggdrasilMacroCall,
        YggdrasilModifiers, YggdrasilNamepath,
    },
};

impl ParseTreeVisitorCompat<'_> for YggdrasilANTLR {
    type Node = YggdrasilAntlrParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.dirty
    }
}

/// Convert weakly typed ast to strongly typed ast
impl YggdrasilAntlrVisitor<'_> for YggdrasilANTLR {
    fn visit_define_grammar(&mut self, ctx: &Define_grammarContext<'_>) {
        self.grammar.name = YggdrasilIdentifier::take(ctx.identifier(0)).unwrap();
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
    fn visit_define_token(&mut self, ctx: &Define_tokenContext<'_>) {
        if let Some(s) = BatchClass::take_one(ctx) {
            for e in s.classes {
                self.grammar.insert(e);
            }
        }
    }
}
