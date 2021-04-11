use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use tree_sitter::{LanguageError, Range};

pub type Result<T> = std::result::Result<T, YGGError>;

#[derive(Debug)]
pub enum YGGError {
    IOError { error: String},
    LanguageError { error: String },
    TextDecodeFailed { error: String },
    NodeMissing { name: String, range: Range },
    InfoMissing { text: String },
    InitializationFailed,
    UnknownError
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

impl From<std::str::Utf8Error> for YGGError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::TextDecodeFailed { error: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for YGGError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::TextDecodeFailed { error: e.to_string() }
    }
}
impl From<std::io::Error> for YGGError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError {error: e.to_string()}
    }
}

impl From<()> for YGGError {
    fn from(_: ()) -> Self {
        Self::UnknownError
    }
}





impl YGGError {
    pub fn node_missing(name: &str, range: Range) -> Self {
        Self::NodeMissing { name: String::from(name), range }
    }
    pub fn init_fail() -> Self {
        Self::InitializationFailed
    }
    pub fn text_decode_failed(e: impl Into<String>) -> Self {
        Self::TextDecodeFailed { error: e.into() }
    }
    pub fn info_missing(e: impl Into<String>) -> Self {
        Self::InfoMissing { text: e.into() }
    }
}
