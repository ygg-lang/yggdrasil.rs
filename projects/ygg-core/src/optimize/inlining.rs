use super::*;

pub struct InlineRules {
    grammar: GrammarInfo,
    recursion_limit: usize,
}

impl Default for InlineRules {
    fn default() -> Self {
        Self { grammar: Default::default(), recursion_limit: 1024 }
    }
}

impl CodeOptimizer for InlineRules {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        self.grammar = info.clone();
        let mut out = info.clone();
        Validation::Success { value: out, diagnostics: vec![] }
    }
}

impl InlineRules {
    fn inline_node(&mut self, info: &mut YggdrasilExpression, depth: usize) -> Result<(), QError> {
        todo!()
    }
}
