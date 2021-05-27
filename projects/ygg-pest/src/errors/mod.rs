mod error_custom;
mod error_gen;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, YGGError>;

#[rustfmt::skip]
#[derive(Error, Debug)]
pub enum YGGError {
    IOError { #[backtrace] source: std::io::Error },
    FormatError { #[backtrace] source: std::fmt::Error },
    LanguageError { error: String },
    NodeMissing { error: String },
    InfoMissing { error: String },
    Unreachable,
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}
