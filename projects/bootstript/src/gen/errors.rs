use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use tree_sitter::LanguageError;

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
pub enum MyError {
    LanguageError { error: String },
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MyError {}

impl From<LanguageError> for MyError {
    fn from(e: LanguageError) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}
