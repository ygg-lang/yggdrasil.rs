use super::*;

impl<'i> RustWriteAST<'i> {
    pub fn token_name(&self) -> String {
        self.grammar.name.text.to_upper_camel_case()
    }
    pub fn language_id(&self) -> String {
        self.grammar.name.text.to_snake_case()
    }
}
