#![feature(try_trait_v2)]
// #![forbid(missing_docs)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use self::errors::{YError, YErrorKind, YResult};
pub use crate::{
    cst_mode::CstNode,
    managers::{
        language_manager::{LanguageID, LanguageManager},
        node_manager::{NodeID, NodeManager, NodeType},
        parse_context::CstContext,
        text_manager::TextManager,
    },
    records::text_index::*,
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
