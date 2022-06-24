#![feature(try_trait_v2)]
// #![forbid(missing_docs)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;

pub use crate::{
    ast_mode::YState,
    cst_mode::CSTNode,
    managers::{
        language_manager::{LanguageID, LanguageManager},
        node_manager::{NodeID, NodeManager, NodeType},
        text_manager::TextManager,
    },
    records::text_index::*,
    results::{
        Parsed,
        SResult::{self, Pending, Stop},
        StopBecause,
    },
};

pub use self::errors::{YError, YErrorKind, YResult};

///
pub mod ast_mode;
mod errors;
mod managers;
pub(crate) mod records;
mod results;
///
pub mod traits;

// pub(crate) mod text_store;
pub mod cst_mode;
