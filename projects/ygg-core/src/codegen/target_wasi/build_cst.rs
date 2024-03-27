use super::*;

impl<'i> RustWriteCST<'i> {
    pub fn token_name(&self) -> String {
        format!("{}Token", self.grammar.name.text.to_upper_camel_case())
    }
    pub fn token_variant(&self, rule: &GrammarRule) -> String {
        format!("{}Token::{}", self.grammar.name.text.to_upper_camel_case(), rule.name.text.to_upper_camel_case())
    }
}
