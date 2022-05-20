#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use crate::records::{cst_node::CSTNode, text_index::*};

pub(crate) mod macros;
pub(crate) mod records;
pub mod results;
pub mod str2ast;
pub mod traits;
