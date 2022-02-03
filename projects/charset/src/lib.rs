#![doc = include_str!("../Readme.md")]

use std::ops::Range;

use serde::{Deserialize, Serialize};
pub use ucd_trie::{TrieSetOwned, TrieSetSlice};

pub mod builtin;
mod insert;
mod utils;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CharacterSet(TrieSetOwned);

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharacterInsert {
    pub fast: bool,
    pub range: Range<u32>,
}
