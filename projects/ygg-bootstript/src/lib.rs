#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]

#[allow(unused)]
#[allow(dead_code)]
pub mod codegen;
#[allow(dead_code)]
#[allow(unused)]
mod ygg;

pub use self::ygg::{ast, Result, YGGError, SyntaxKind};
pub use tree_sitter::{Parser, Tree};
pub use tree_sitter_ygg::language;
pub use lsp_types;
