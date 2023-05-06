#![feature(lazy_cell)]

// pub(crate) mod antlr;

// mod ast;
pub mod bootstrap;

pub use bootstrap::BootstrapParser;

pub use yggdrasil_rt::{YggdrasilError, YggdrasilNode, YggdrasilRule};
// mod traits;

// pub use self::ast::YggdrasilANTLR;
