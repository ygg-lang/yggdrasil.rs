#![cfg_attr(nightly, feature(try_trait_v2))]
#![forbid(missing_docs)]
#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use crate::{
    cst_mode::{CSTNode, NodeType},
    records::text_index::*,
    results::{Parsed, YResult, YYResult},
};

// pub use self::errors::{YError, YErrorKind};

///
pub mod ast_mode;
// mod errors;
pub(crate) mod records;
mod results;
///
pub mod traits;

// pub(crate) mod text_store;
pub mod cst_mode;
