use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub range: Range<usize>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SymbolAlias {
    pub name: String,
    pub alias: String,
    pub range: Range<usize>,
}
