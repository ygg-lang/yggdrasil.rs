use crate::grammar::GrammarInfo;
use std::collections::HashSet;
use yggdrasil_error::{Validate, Validation};

/// Indicates what kind of structure a rule should generate
///
/// ```ygg
/// class A {
///     a: (x b:y+)?
/// }
/// ```
pub trait FieldDescriptor {
    fn visit_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>);
    fn visit_field_count(&self, buffer: &mut HashSet<String>);
}

pub trait CodeOptimizer {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo>;
}

pub trait CodeGenerator {
    type Output;
    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output>;
}

impl GrammarInfo {
    pub fn optimize(&self, mut pass: Vec<Box<dyn CodeOptimizer>>) -> Validation<GrammarInfo> {
        let mut errors = vec![];
        let mut current = self.clone();
        for co in pass.iter_mut() {
            current = co.optimize(self).validate(&mut errors)?;
        }
        Validation::Success { value: current, diagnostics: errors }
    }
    pub fn generate<T>(&self, mut pass: T) -> Validation<<T as CodeGenerator>::Output>
    where
        T: CodeGenerator,
    {
        pass.generate(self)
    }
}
