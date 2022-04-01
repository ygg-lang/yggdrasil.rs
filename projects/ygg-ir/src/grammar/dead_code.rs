use std::collections::HashSet;

use crate::rule::node::ExpressionKind;
use yggdrasil_error::{Diagnostic, YggdrasilResult};

use crate::traits::CodeOptimizer;

use super::*;

pub struct DecodeEliminator {
    used: HashSet<String>,
    new: HashSet<String>,
    unvisited: HashSet<String>,
}

impl CodeOptimizer for DecodeEliminator {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo> {
        self.find_entry(info);
        self.find_unvisited();
        while !self.unvisited.is_empty() {
            self.find_unvisited();
        }
        let rules = self.find_needed(info);
        self.clear();
        Ok(Diagnostic { success: GrammarInfo { rules, ..info.clone() }, errors: vec![] })
    }
}

impl DecodeEliminator {
    fn find_entry(&mut self, info: &GrammarInfo) {
        for (_, rule) in &info.rules {
            if rule.keep || rule.entry {
                self.new.insert(rule.name.to_owned());
            }
        }
    }
    fn find_unvisited(&mut self) {
        for rule in self.new.difference(&self.used) {
            self.unvisited.insert(rule.to_owned())
        }
    }
    fn find_needed(&self, info: &GrammarInfo) -> BTreeMap<String, GrammarRule> {
        todo!()
    }
    fn clear(&mut self) {
        self.used.clear();
        self.new.clear();
    }
}

impl GrammarRule {
    pub fn get_field_names(&self, buffer: &mut HashSet<&String>) {
        self.body.get_fields(buffer)
    }
}

impl ExpressionKind {
    pub fn get_field_names(&self, buffer: &mut HashSet<&String>) {
        match self {
            ExpressionKind::Choice(e) => e.get_fields(buffer),
            ExpressionKind::Concat(e) => e.get_field_names(buffer),
            ExpressionKind::Unary(e) => e.get_fields(buffer),
            ExpressionKind::Data(_) => {}
            ExpressionKind::Rule(e) => buffer.insert(&e.name),
        }
    }
}

impl ConcatExpression {
    pub fn get_field_names(&self, buffer: &mut HashSet<&String>) {}
}

impl ChoiceExpression {
    pub fn get_field_names(&self, buffer: &mut HashSet<&String>) {
        self.inner.iter().for_each(|f| f.get_field_names(buffer))
    }
}

impl ConcatExpression {
    pub fn get_field_names(&self, buffer: &mut HashSet<&String>) {
        self.sequence.iter().for_each(|f| f.get_field_names(buffer))
    }
}
