use yggdrasil_error::Validation;
use yggdrasil_ir::rule::GrammarBody;

use super::*;

/// According to atomic rules, insert necessary ignore nodes
///
/// | Before | After |
/// | :-: | :-: |
/// | `A B` | `A ~ B` |
/// | `A \| B` | `A \| B` |
/// | `A?` | `A?` |
/// | `A*` | `(~ A)*` |
/// | `A+` | `(~ A)+` |
pub struct InlineRules {
    grammar: GrammarInfo,
}

impl Default for InlineRules {
    fn default() -> Self {
        Self { grammar: Default::default() }
    }
}

impl CodeOptimizer for InlineRules {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let mut errors = vec![];
        self.grammar = info.clone();
        let mut out = info.clone();
        for rule in self.grammar.rules.values() {
            match &rule.body {
                // TODO: nested inline classes
                GrammarBody::Class { .. } => {}
                GrammarBody::Union { branches, .. } => {
                    for (index, branch) in branches.iter().enumerate() {
                        let variant = branch.as_variant(rule.name.text.as_str(), index);
                        match variant.additional_rule {
                            Some(s) => match out.register(s) {
                                Ok(_) => continue,
                                Err(e) => errors.push(e),
                            },
                            None => continue,
                        }
                    }
                }
                GrammarBody::Climb { .. } => {}
            }
        }
        Validation::Success { value: out, diagnostics: errors }
    }
}

impl InlineRules {}
