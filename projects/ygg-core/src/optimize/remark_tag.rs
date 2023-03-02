use yggdrasil_error::Validation;
use super::*;

pub struct RemarkTags {}

impl CodeOptimizer for RemarkTags {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let _ = info;
        let e = ChoiceExpression { branches: Default::default() };
        self.fuse_choice(&e);
        todo!()
    }
}

impl RemarkTags {
    fn fuse_choice(&mut self, choice: &ChoiceExpression) -> Validation<ChoiceExpression> {
        let mut errors = vec![];
        for branch in &choice.branches {}
        Validation::Success { value: ChoiceExpression { branches: Default::default() }, diagnostics: errors }
    }
}
