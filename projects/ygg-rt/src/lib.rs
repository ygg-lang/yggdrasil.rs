#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub(crate) mod macros;
pub(crate) mod records;
pub mod traits;
pub use self::records::text_index::*;
#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;
