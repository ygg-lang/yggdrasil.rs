#![feature(lazy_cell)]

#[cfg(debug_assertions)]
pub mod antlr;

// mod ast;
pub mod bootstrap;

mod helpers;

pub use bootstrap::BootstrapParser;

pub use yggdrasil_rt::{YggdrasilError, YggdrasilNode, YggdrasilRule};
// mod traits;

// pub use self::ast::YggdrasilANTLR;
pub use crate::helpers::TakeAnnotations;
