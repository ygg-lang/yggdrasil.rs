#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub(crate) mod macros;
pub(crate) mod records;
pub(crate) mod traits;
pub use self::records::{text_index::*, text_store::TextStore};
#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;
