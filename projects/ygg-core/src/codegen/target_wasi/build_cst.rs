use super::*;

impl<'i> RustWriteCST<'i> {
    pub fn token_name(&self) -> String {
        self.grammar.name.text.to_upper_camel_case()
    }
}
