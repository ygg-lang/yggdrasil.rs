#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]

mod codegen;
mod debug;
#[allow(unused)]
mod ygg;

pub use tree_sitter::{Parser, Tree};
pub use self::ygg::{Result,ast};
pub use tree_sitter_yg::language;
pub use self::ygg::SyntaxKind;
