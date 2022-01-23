#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;

pub use self::errors::{
    diagnostic::{Diagnostic, DiagnosticLevel},
    YggdrasilError, YggdrasilErrorKind, YggdrasilResult,
};
pub use url::Url;
mod error_3rd;
mod errors;
