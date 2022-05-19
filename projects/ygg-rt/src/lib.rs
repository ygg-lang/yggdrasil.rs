#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use crate::records::{cst_node::CSTNode, text_index::*};

pub(crate) mod macros;
pub mod parser;
pub(crate) mod records;
pub mod traits;
