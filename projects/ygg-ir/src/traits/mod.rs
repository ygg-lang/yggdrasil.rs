use crate::*;
use yggdrasil_error::{Diagnostic, YggdrasilResult};

pub trait CodeOptimizer {
    fn optimize(&self, info: &mut GrammarInfo) -> YggdrasilResult;
}

pub trait CodeGenerator<T> {
    fn generate(&self, info: &GrammarInfo) -> YggdrasilResult<T>;
}

impl GrammarInfo {
    pub fn optimize(&mut self, pass: &[impl CodeOptimizer]) -> YggdrasilResult {
        let mut errors = vec![];
        for opt in pass {
            errors.extend(opt.optimize(self)?.errors);
        }
        Ok(Diagnostic { success: (), errors })
    }
    pub fn codegen<T>(&self, pass: impl CodeGenerator<T>) -> YggdrasilResult {
        pass.generate(self)
    }
}
