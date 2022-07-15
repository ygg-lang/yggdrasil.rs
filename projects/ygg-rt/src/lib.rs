#![feature(try_blocks)]
#![feature(once_cell)]
// #![forbid(missing_docs)]
#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub use self::errors::{YError, YErrorKind, YResult};
pub use crate::{
    cst_mode::{ConcreteNode, ConcreteTree},
    managers::text_manager::{TextManager, TEXT_MANAGER},
    records::text_index::*,
    traits::{AstNode, NodeType},
};
pub use indextree::NodeId;
#[cfg(feature = "lsp")]
pub use lsp_types;
pub use pex::{
    ParseResult::{self, Pending, Stop},
    ParseState, Parsed, StopBecause,
};
pub use url::Url;

///
mod errors;
mod managers;
pub(crate) mod records;
///
pub mod traits;

// pub(crate) mod text_store;
pub mod cst_mode;
