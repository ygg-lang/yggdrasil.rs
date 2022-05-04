#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub use diagnostic::{DiagnosticLevel, FileID, TextStorage};

pub use self::errors::{Validation, YggdrasilError, YggdrasilErrorKind, YggdrasilResult};

mod error_3rd;
mod errors;
