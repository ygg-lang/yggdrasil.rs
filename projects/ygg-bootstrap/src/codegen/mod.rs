#[cfg(feature = "peginator")]
mod target_peg;
#[cfg(feature = "pest")]
mod target_pest;
#[cfg(feature = "railroad")]
pub(crate) mod target_railroad;
#[cfg(feature = "railroad")]
pub use self::target_railroad::Railroad;
#[cfg(feature = "tree-sitter")]
pub(crate) mod target_tree_sitter;

#[cfg(feature = "peginator")]
pub use self::target_peg::as_peg;
