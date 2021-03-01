#![feature(once_cell)]

mod debug;
#[allow(unused)]
mod ygg;
mod codegen;

pub use tree_sitter::{Parser, Tree};

pub use tree_sitter_yg::language;

pub use self::ygg::SyntaxKind;


