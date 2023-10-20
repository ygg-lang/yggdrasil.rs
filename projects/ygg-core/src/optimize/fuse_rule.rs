use super::*;
use yggdrasil_error::Validation;

pub struct FusionRules {}

impl CodeOptimizer for FusionRules {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let _ = info;
        let e = ChoiceExpression { branches: Default::default() };
        self.fuse_choice(&e);
        todo!()
    }
}

impl FusionRules {
    fn fuse_choice(&mut self, choice: &ChoiceExpression) -> Validation<ChoiceExpression> {
        let errors = vec![];
        for branch in &choice.branches {}
        Validation::Success { value: ChoiceExpression { branches: Default::default() }, diagnostics: errors }
    }
}
