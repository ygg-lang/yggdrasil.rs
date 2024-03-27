use super::*;

impl<'i> RustWriteAST<'i> {
    pub fn token_name(&self) -> String {
        format!("{}Token", self.grammar.name.text).to_upper_camel_case()
    }
    pub fn node_trait(&self, rule: &GrammarRule) -> String {
        format!("Guest{}{}Node", self.grammar.name.text, rule.name.text).to_upper_camel_case()
    }
    pub fn rule_variant(&self, rule: &GrammarRule) -> String {
        rule.name.text.to_upper_camel_case()
    }
    pub fn node_native(&self, rule: &GrammarRule) -> String {
        format!("{}Native", rule.name.text).to_upper_camel_case()
    }
    pub fn host_name(&self) -> String {
        format!("{}Host", self.grammar.name.text).to_upper_camel_case()
    }
    pub fn node_wasi(&self, rule: &GrammarRule) -> String {
        format!("{}{}Node", self.grammar.name.text, rule.name.text).to_upper_camel_case()
    }
    pub fn language_id(&self) -> String {
        self.grammar.name.text.to_snake_case()
    }
    pub fn language_name(&self) -> String {
        self.grammar.name.text.clone()
    }
}
