use super::*;
use yggdrasil_error::Validation;

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
        for rule in out.rules.values_mut() {
            let is_union = rule.is_union();
            match &mut rule.body {
                Some(s) => match &mut s.body {
                    ExpressionBody::Choice(c) if is_union => {
                        for x in c.branches.iter_mut() {
                            match self.refine_node(x) {
                                Ok(_) => {}
                                Err(e) => errors.push(e),
                            }
                        }
                    }
                    _ => match self.refine_node(s) {
                        Ok(_) => {}
                        Err(e) => errors.push(e),
                    },
                },
                None => {}
            }
        }
        Validation::Success { value: out, diagnostics: errors }
    }
}

impl RefineRules {
    fn refine_node(&mut self, node: &mut YggdrasilExpression) -> Result<(), YggdrasilError> {
        match &mut node.body {
            ExpressionBody::Choice(v) => {
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
