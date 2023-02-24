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
        for node in out.rules.values_mut() {
            match &mut node.body {
                Some(s) => match self.refine_node(s) {
                    Ok(_) => {}
                    Err(e) => errors.push(e),
                },
                None => {}
            }
        }
        Validation::Success { value: out, diagnostics: errors }
    }
}

impl RefineRules {
    fn refine_node(&mut self, node: &mut YggdrasilExpression) -> Result<(), QError> {
        match &mut node.kind {
            ExpressionKind::Choice(v) => {
                for child in v.branches.iter_mut() {
                    self.refine_node(child)?;
                }
                let (mut head, rest) = v.split();
                for term in rest {
                    head |= term;
                }
                *node = head
            }
            ExpressionKind::Concat(v) => {
                for child in v.sequence.iter_mut() {
                    self.refine_node(child)?;
                }
                let (mut head, rest) = v.split();
                self.refine_node(&mut head)?;
                for mut term in rest {
                    self.refine_node(&mut term)?;
                    head &= term;
                }
                *node = head
            }
            ExpressionKind::Unary(v) => {}
            _ => {}
        }
        Ok(())
    }
}