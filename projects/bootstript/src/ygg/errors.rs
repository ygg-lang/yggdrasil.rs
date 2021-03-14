use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    str::Utf8Error,
};
use tree_sitter::{LanguageError, Range};

pub type Result<T> = std::result::Result<T, YGGError>;

#[derive(Debug)]
pub enum YGGError {
    LanguageError { error: String },
    TextError { error: String },
    NodeMissing { name: String, range: Range },
    InitializationFailed
}

impl Display for YGGError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for YGGError {}

impl From<LanguageError> for YGGError {
    fn from(e: LanguageError) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl From<Utf8Error> for YGGError {
    fn from(e: Utf8Error) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl YGGError {
    pub fn node_missing(name: &str, range: Range) -> Self {
        Self::NodeMissing { name: String::from(name), range }
    }
    pub fn init_fail()-> Self {
        Self::InitializationFailed
    }
}
