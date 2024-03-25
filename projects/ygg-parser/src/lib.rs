#![feature(lazy_cell)]

#[cfg(debug_assertions)]
pub mod antlr;

pub mod bootstrap;

mod helpers;
mod states;
mod traits;

pub use crate::{bootstrap::BootstrapParser, helpers::parse_grammar_info};
pub use yggdrasil_rt::{YggdrasilError, YggdrasilNode, YggdrasilRule};
// mod traits;
