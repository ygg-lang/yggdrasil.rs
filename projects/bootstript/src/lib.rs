#![feature(once_cell)]

mod debug;
#[allow(unused)]
mod ygg;
mod codegen;

use tree_sitter::{Parser, Tree};

use tree_sitter_yg::language;

pub use self::ygg::SyntaxKind;


