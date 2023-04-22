use super::*;
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol {
    pub name: String,
    pub range: Range<usize>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolAlias {
    pub name: String,
    pub alias: String,
    pub range: Range<usize>,
}
