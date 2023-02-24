use super::*;

pub struct RefineRules {
    grammar: GrammarInfo,
    recursion_limit: usize,
}

impl Default for RefineRules {
    fn default() -> Self {
        Self { grammar: Default::default(), recursion_limit: 1024 }
    }
}

impl CodeOptimizer for RefineRules {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        self.grammar = info.clone();
        let mut out = info.clone();
        Validation::Success { value: out, diagnostics: vec![] }
    }
}

impl RefineRules {
    fn inline_node(&mut self, info: &mut YggdrasilExpression, depth: usize) -> Result<(), QError> {
        todo!()
    }
}
