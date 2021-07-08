use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

mod error_custom;
mod error_gen;

pub type Result<T> = std::result::Result<T, Error>;

#[rustfmt::skip]
#[derive(Error, Debug)]
pub enum Error {
    IOError { #[from] source: std::io::Error },
    FormatError { #[from] source: std::fmt::Error },
    ParsingError { error: String },
    LanguageError { error: String },
    NodeMissing { error: String },
    NodeTagMissing { error: String },
    InfoMissing { error: String },
    /// Some nodes failed to resolve and are being rolled back
    Unwinding,
    /// A forbidden node encountered
    Unreachable,
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}

impl Error {
    pub fn node_missing(msg: impl Into<String>) -> Error {
        Self::NodeMissing { error: msg.into() }
    }
    pub fn node_tag_missing(msg: impl Into<String>) -> Error {
        Self::NodeTagMissing { error: msg.into() }
    }
    pub fn language_error(msg: impl Into<String>) -> Error {
        Self::LanguageError { error: msg.into() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
