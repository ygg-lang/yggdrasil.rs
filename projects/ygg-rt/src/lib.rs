#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use crate::{
    cst_mode::{CSTNode, NodeType},
    records::text_index::*,
};

pub mod ast_mode;
pub(crate) mod macros;
pub(crate) mod records;
pub mod results;
pub mod traits;

// pub(crate) mod text_store;
pub mod cst_mode;
