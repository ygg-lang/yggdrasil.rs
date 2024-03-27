pub(crate) mod target_railroad;
#[allow(unused)]
mod target_rust;

#[allow(unused)]
pub(crate) mod target_wasi;

pub use self::target_railroad::BuildRailway;
#[cfg(feature = "tree-sitter")]
pub(crate) mod target_vscode;

pub use self::{
    target_rust::{BuildRust, RustModule},
    target_wasi::BuildWasi,
};
