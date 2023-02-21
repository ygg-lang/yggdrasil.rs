use super::*;

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
            // self.emit_expression(&mut rule.body)?;
        }
        Ok(())
    }
}

impl ExpressionNode {}
