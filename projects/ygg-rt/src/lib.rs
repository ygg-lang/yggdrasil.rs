#![feature(try_blocks)]
// #![forbid(missing_docs)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use self::errors::{YError, YErrorKind, YResult};
pub(crate) use crate::cst_mode::CstTyped;
pub use crate::{
    cst_mode::CstNode,
    managers::{
        language_manager::{LanguageID, LanguageManager},
        node_manager::{NodeID, NodeManager},
        parse_context::CstContext,
        text_manager::TextManager,
    },
    records::text_index::*,
    traits::{AstNode, NodeType},
};
pub use pex::{
    ParseResult::{self, Pending, Stop},
    ParseState, Parsed, StopBecause,
};
pub use rand::{rngs::SmallRng, Rng, SeedableRng};

///
mod errors;
mod managers;
pub(crate) mod records;
///
pub mod traits;

// pub(crate) mod text_store;
pub mod cst_mode;
