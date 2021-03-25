#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]

mod codegen;
#[allow(unused)]
mod ygg;

pub use self::ygg::{ast, Result, SyntaxKind};
pub use tree_sitter::{Parser, Tree};
pub use tree_sitter_yg::language;
