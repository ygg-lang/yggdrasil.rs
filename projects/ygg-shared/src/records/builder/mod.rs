use crate::{errors::Error, records::CSTNode, traits::CSTParser, Result};

pub struct CSTBuilder {
    pub error: Vec<Error>,
}

pub struct ASTBuilder {
    pub input: String,
    pub error: Vec<Error>,
}

impl Default for CSTBuilder {
    fn default() -> Self {
        Self { error: vec![] }
    }
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self { input: "".to_string(), error: vec![] }
    }
}

impl CSTBuilder {
    pub fn parse<L: CSTParser<R>, R>(&self, parser: &mut L, input: &str) -> Result<CSTNode<R>> {
        parser.parse(input)
    }
}

impl ASTBuilder {}
