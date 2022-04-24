use std::mem::take;

use indexmap::IndexMap;

use yggdrasil_error::{Diagnostic, YggdrasilError, YggdrasilResult};

use crate::{CodeOptimizer, ExpressionKind, ExpressionNode, FunctionRule, GrammarInfo, GrammarRule};

pub struct EmitFunction {
    functions: IndexMap<String, FunctionRule>,
    errors: Vec<YggdrasilError>,
}

impl Default for EmitFunction {
    fn default() -> Self {
        Self { functions: Default::default(), errors: vec![] }
    }
}

impl CodeOptimizer for EmitFunction {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo> {
        let mut rules = info.rules.clone();
        self.functions = info.functions.clone();
        self.emit(&mut rules)?;
        // Reset Progress
        let grammar = GrammarInfo { rules: take(&mut rules), functions: Default::default(), ..info.clone() };
        let errors = take(&mut self.errors);
        Ok(Diagnostic { success: grammar, errors })
    }
}

impl EmitFunction {
    fn emit(&mut self, rules: &mut IndexMap<String, GrammarRule>) -> Result<(), YggdrasilError> {
        for (_, rule) in rules.iter_mut() {
            self.emit_expression(&mut rule.body)?;
        }
        Ok(())
    }

    fn emit_expression(&mut self, e: &mut ExpressionNode) -> Result<(), YggdrasilError> {
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
