use super::*;

impl<'i> RustWriteMain<'i> {
    pub fn node_native(&self, rule: &GrammarRule) -> String {
        format!("{}Native", rule.name.text).to_upper_camel_case()
    }
    pub fn token_name(&self) -> String {
        format!("{}Token", self.grammar.name.text).to_upper_camel_case()
    }
    pub fn host_name(&self) -> String {
        format!("{}Host", self.grammar.name.text).to_upper_camel_case()
    }
    pub fn language_id(&self) -> String {
        self.grammar.name.text.to_snake_case()
    }
}
