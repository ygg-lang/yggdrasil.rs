use crate::{
    data::{DataKind, RuleReference},
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionKind, UnaryExpression, YggdrasilExpression},
    rule::GrammarRule,
    FunctionExpression,
};
use diagnostic_quick::Validation;
use std::collections::{BTreeMap, HashSet};

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
    pub fn optimize(&self, mut pass: Vec<impl CodeOptimizer>) -> Validation<GrammarInfo> {
        let mut errors = vec![];
        let mut out = GrammarInfo::default();
        for co in pass.iter_mut() {
            match co.optimize(self) {
                Validation::Success { value, diagnostics } => {
                    out = value;
                    errors.extend(diagnostics.into_iter())
                }
                Validation::Failure { fatal, diagnostics } => {
                    errors.extend(diagnostics.into_iter());
                    return Validation::Failure { fatal, diagnostics: errors };
                }
            }
        }
        Validation::Success { value: out, diagnostics: errors }
    }
    pub fn generate<T>(&self, mut pass: T) -> Validation<<T as CodeGenerator>::Output>
    where
        T: CodeGenerator,
    {
        pass.generate(self)
    }
}
