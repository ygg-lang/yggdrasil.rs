use std::collections::HashSet;

use yggdrasil_error::{Diagnostic, YggdrasilResult};

use crate::*;

mod field_descriptor;

pub enum FieldCount {
    Optional(RuleReference),
    One(RuleReference),
    Many(RuleReference),
}

pub trait FieldDescriptor {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>);
    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>);
}

pub trait CodeOptimizer {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo>;
}

pub trait CodeGenerator {
    type Output;
    fn generate(&mut self, info: &GrammarInfo) -> YggdrasilResult<Self::Output>;
}

impl GrammarInfo {
    pub fn optimize(&self, mut pass: Vec<impl CodeOptimizer>) -> YggdrasilResult<GrammarInfo> {
        let mut errors = vec![];
        let mut out = GrammarInfo::default();
        for co in pass.iter_mut() {
            let step = co.optimize(self)?;
            out = step.success;
            errors.extend(step.errors);
        }
        Ok(Diagnostic { success: out, errors })
    }
    pub fn generate<T>(&self, mut pass: T) -> YggdrasilResult<<T as CodeGenerator>::Output>
    where
        T: CodeGenerator,
    {
        pass.generate(self)
    }
}
