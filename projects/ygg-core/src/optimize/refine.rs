use yggdrasil_error::{Validate, Validation};
use yggdrasil_ir::rule::{GrammarBody, YggdrasilIdentifier, YggdrasilVariant};

use super::*;

pub struct RefineRules {
    grammar: GrammarInfo,
}

impl Default for RefineRules {
    fn default() -> Self {
        Self { grammar: Default::default() }
    }
}

impl CodeOptimizer for RefineRules {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let mut errors = vec![];
        self.grammar = info.clone();
        let mut out = info.clone();
        for rule in self.grammar.rules.values() {
            match &rule.body {
                GrammarBody::Class { .. } => {}
                GrammarBody::Union { branches, .. } => {
                    for (index, branch) in branches.iter().enumerate() {
                        match branch.as_variant(rule.name.text.as_str(), index) {
                            YggdrasilVariant::Reference { .. } => {}
                            YggdrasilVariant::Additional { rule, .. } => match out.register(rule) {
                                Ok(_) => {}
                                Err(e) => errors.push(e),
                            },
                        }
                    }
                }
                GrammarBody::Climb { .. } => {}
            }
        }
        for rule in out.rules.values_mut() {
            match &mut rule.body {
                GrammarBody::Class { term } => {
                    self.refine_node(term).recover(&mut errors);
                }
                GrammarBody::Union { branches, refined } => {
                    let mut mapping = IndexMap::default();
                    for (index, branch) in branches.iter().enumerate() {
                        match branch.as_variant(rule.name.text.as_str(), index) {
                            YggdrasilVariant::Reference { variant } => {
                                mapping.insert(variant.text.clone(), rule.name.text.clone());
                            }
                            YggdrasilVariant::Additional { variant, rule } => {
                                mapping.insert(variant.text.clone(), rule.name.text.clone());
                            }
                        }
                    }
                    *refined = mapping;
                }
                GrammarBody::Climb { .. } => {}
            }
        }
        Validation::Success { value: out, diagnostics: errors }
    }
}

impl RefineRules {
    fn refine_node(&mut self, node: &mut YggdrasilExpression) -> Result<(), YggdrasilError> {
        match &mut node.body {
            ExpressionBody::Choice(v) => {
                // if v.branches.len() == 1 {
                //     let head = v.branches.pop().unwrap();
                //     *node = head;
                // }

                for child in v.branches.iter_mut() {
                    self.refine_node(child)?;
                }
                let (mut head, rest) = v.split();
                for term in rest {
                    head |= term.clone();
                }
                *node = head
            }
            ExpressionBody::Concat(v) => {
                // if v.sequence.len() == 1 {
                //     let head = v.sequence.pop().unwrap();
                //     *node = head;
                // }
                for child in v.sequence.iter_mut() {
                    self.refine_node(child)?;
                }
                let (mut head, rest) = v.split();
                for term in rest {
                    head &= term.clone();
                }
                *node = head
            }
            ExpressionBody::Unary(v) => {
                // TODO: marge operators,
                // ** -> *
                // ?* -> *
                self.refine_node(&mut v.base)?
            }
            _ => {}
        }
        Ok(())
    }
}
