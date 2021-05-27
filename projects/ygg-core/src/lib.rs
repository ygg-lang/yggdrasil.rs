#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(trivial_bounds)]
#![feature(async_closure)]

pub mod codegen;
#[allow(dead_code)]
#[allow(unused)]
mod ygg;

mod target_pest;
mod target_tree_sitter;

pub mod manager;

pub use self::{
    manager::{FILE_MANAGER, HINT_MANAGER},
    ygg::{ast, Result, SyntaxKind, YGGError},
};
pub use tree_sitter::{Parser, Tree};
pub use tree_sitter_ygg::language;
