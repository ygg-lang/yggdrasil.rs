use super::*;
use std::hash::{Hash, Hasher};

impl Eq for SymbolPath {}
impl PartialEq<Self> for SymbolPath {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
impl Hash for SymbolPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}

impl Eq for Symbol {}
impl PartialEq<Self> for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl Eq for StringLiteral {}
impl PartialEq<Self> for StringLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Hash for StringLiteral {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}
