use super::*;
use yggdrasil_ir::rule::GrammarBody;

impl<'i> WasiWriteWit<'i> {
    pub fn language_id(&self) -> String {
        self.grammar.name.text.to_kebab_case()
    }

    pub fn rule_name(&self, rule: &GrammarRule) -> String {
        rule.name.text.to_kebab_case()
    }

    pub fn variants(&self) -> Vec<GrammarRule> {
        self.grammar.rules.values().filter(|x| x.is_union()).cloned().collect()
    }
    pub fn variant_items(&self, rule: &GrammarRule) -> Vec<(String, String)> {
        match &rule.body {
            GrammarBody::Union { refined, .. } => {
                refined.iter().map(|(key, value)| (key.to_kebab_case(), value.to_kebab_case())).collect()
            }
            _ => panic!(""),
        }
    }

    pub fn records(&self) -> Vec<GrammarRule> {
        self.grammar.rules.values().filter(|x| x.is_class()).cloned().collect()
    }

    pub fn token_type(&self) -> Vec<(String, String)> {
        self.grammar.rules.iter().map(|(key, value)| (key.to_kebab_case(), value.name.text.to_kebab_case())).collect()
    }
}
