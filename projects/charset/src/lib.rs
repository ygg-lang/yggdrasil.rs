#![doc = include_str!("../Readme.md")]

use std::ops::Range;

mod utils;
mod insert;
pub mod builtin;

/// Some characters appear significantly more frequently than others, and you need to quickly search for this high-frequency character set
/// Such as ascii character set
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CharacterSet {
    #[cfg(debug_assertions)]
    pub optimized: bool,
    /// Sort by the frequency of use
    pub fast: Vec<Range<u32>>,
    /// Sort by smaller value
    pub common: Vec<Range<u32>>,
}

pub struct CharacterInsert {
    pub fast: bool,
    pub range: Range<u32>,
}