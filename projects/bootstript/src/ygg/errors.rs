use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use tree_sitter::LanguageError;
use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
pub enum MyError {
    LanguageError { error: String },
    TextError{
        error: String
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for MyError {}

impl From<LanguageError> for MyError {
    fn from(e: LanguageError) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl From<Utf8Error> for MyError {
    fn from(e: Utf8Error) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}
