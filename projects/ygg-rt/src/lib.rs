#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub(crate) mod macros;
pub(crate) mod records;
pub mod traits;
pub use self::records::text_index::*;
pub use crate::records::cst_node::{CSTNode, CSTArena};
#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;
