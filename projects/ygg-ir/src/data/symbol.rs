use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub range: Range<usize>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolAlias {
    pub name: String,
    pub alias: String,
    pub range: Range<usize>,
}
