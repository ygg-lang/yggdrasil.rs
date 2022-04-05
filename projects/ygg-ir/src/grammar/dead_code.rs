use std::collections::HashSet;

use yggdrasil_error::{Diagnostic, YggdrasilResult};

use crate::traits::CodeOptimizer;

use super::*;

pub struct DeadCodeEliminator {
    used: HashSet<String>,
    new: HashSet<String>,
    unvisited: HashSet<String>,
}

impl Default for DeadCodeEliminator {
    fn default() -> Self {
        Self { used: Default::default(), new: Default::default(), unvisited: Default::default() }
    }
}

impl CodeOptimizer for DeadCodeEliminator {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo> {
        self.find_entry(info);
        self.find_unvisited();
        while !self.unvisited.is_empty() {
            for rule in &self.unvisited {
                match info.rules.get(rule) {
                    Some(s) => {
                        let mut new = HashSet::new();
                        s.get_field_names(&mut new);
                        self.new.extend(new.iter().map(|s| s.to_string()))
                    }
                    None => {
                        #[cfg(debug_assertions)]
                        {
                            panic!("Undefined rule {}", rule)
                        }
                    }
                }
            }
            self.find_unvisited();
        }
        let rules = self.find_needed(info);
        self.clear();
        Ok(Diagnostic { success: GrammarInfo { rules, ..info.clone() }, errors: vec![] })
    }
}

impl DeadCodeEliminator {
    fn find_entry(&mut self, info: &GrammarInfo) {
        for (_, rule) in &info.rules {
            if rule.keep || rule.entry {
                self.new.insert(rule.name.to_owned());
            }
        }
    }
    fn find_unvisited(&mut self) {
        for rule in self.new.difference(&self.used) {
            self.unvisited.insert(rule.to_owned());
        }
        self.new.clear();
    }
    fn find_needed(&self, info: &GrammarInfo) -> BTreeMap<String, GrammarRule> {
        info.rules.iter().filter(|r| self.used.contains(r.0)).map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    fn clear(&mut self) {
        self.used.clear();
        self.new.clear();
    }
}
