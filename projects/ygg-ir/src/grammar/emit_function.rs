use std::mem::take;
use diagnostic_quick::{QError, QResult, Validation};

use indexmap::IndexMap;


use crate::{CodeOptimizer, ExpressionKind, ExpressionNode, FunctionRule, GrammarInfo, GrammarRule};

pub struct EmitFunction {
    functions: IndexMap<String, FunctionRule>,
    errors: Vec<QError>,
}

impl Default for EmitFunction {
    fn default() -> Self {
        Self { functions: Default::default(), errors: vec![] }
    }
}

impl CodeOptimizer for EmitFunction {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let mut rules = info.rules.clone();
        self.functions = info.functions.clone();
        match self.emit(&mut rules) {
            Ok(_) => {}
            Err(e) => return Validation::Failure { fatal: e, diagnostics: vec![] },
        }
        // Reset Progress
        let grammar = GrammarInfo { rules: take(&mut rules), functions: Default::default(), ..info.clone() };
        let errors = take(&mut self.errors);
        Validation::Success { value: grammar, diagnostics: errors }
    }
}

impl EmitFunction {
    fn emit(&mut self, rules: &mut IndexMap<String, GrammarRule>) -> QResult {
        for (_, rule) in rules.iter_mut() {
            self.emit_expression(&mut rule.body)?;
        }
        Ok(())
    }

    fn emit_expression(&mut self, e: &mut ExpressionNode) -> QResult {
        match &e.kind {
            ExpressionKind::Function(_) => {}
            ExpressionKind::Choice(_) => {}
            ExpressionKind::Concat(_) => {}
            ExpressionKind::Unary(_) => {}
            ExpressionKind::Rule(_) => {}
            ExpressionKind::Data(_) => {}
        }
        Ok(())
    }
}

impl ExpressionNode {}
