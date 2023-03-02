use crate::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionKind, UnaryExpression, YggdrasilExpression},
    rule::GrammarRule,
    FunctionExpression,
};
use std::collections::{BTreeMap, HashSet};
use yggdrasil_error::Validation;

mod field_descriptor;

pub type FieldMap = BTreeMap<String, FieldCount>;

#[derive(Debug)]
pub enum FieldCount {
    Optional,
    One,
    Many,
}

pub enum FieldCount2 {
    Optional(RuleReference),
    One(RuleReference),
    Many(RuleReference),
}

pub trait FieldDescriptor {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>);
    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount2>);
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
            match co.optimize(self) {
                Validation::Success { value, diagnostics } => {
                    current = value;
                    errors.extend(diagnostics.into_iter())
                }
                Validation::Failure { fatal, diagnostics } => {
                    errors.extend(diagnostics.into_iter());
                    return Validation::Failure { fatal, diagnostics: errors };
                }
            }
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
