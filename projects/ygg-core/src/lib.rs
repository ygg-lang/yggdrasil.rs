#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(trivial_bounds)]
#![feature(async_closure)]

pub use tree_sitter::{Parser, Tree};

pub use tree_sitter_ygg::language;

pub use self::{
    manager::{FILE_MANAGER, HINT_MANAGER},
    ygg::{ast, Result, SyntaxKind, YGGError},
};

pub mod codegen;
#[allow(dead_code)]
#[allow(unused)]
mod ygg;

pub mod manager;

