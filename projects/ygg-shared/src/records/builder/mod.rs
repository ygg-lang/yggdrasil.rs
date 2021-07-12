use crate::errors::Error;

pub struct CSTBuilder {
    pub error: Vec<Error>
}

pub struct ASTBuilder {
   pub input: String,
   pub error: Vec<Error>
}

impl Default for CSTBuilder {
    fn default() -> Self {
        Self {
            error: vec![]
        }
    }
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            error: vec![]
        }
    }
}

impl CSTBuilder {
    pub fn parse<L>( input: &str) {
        L
    }
}

impl ASTBuilder {

}