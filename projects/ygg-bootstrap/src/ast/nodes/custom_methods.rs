use super::*;

impl Expression {
    pub fn range(&self) -> (usize, usize) {
        match self {
            Self::Data(e) => e.range(),
            Self::Concat { lhs, rhs, .. } => (lhs.range().0, rhs.range().1),
            Self::Choice { lhs, rhs } => (lhs.range().0, rhs.range().1),
            Self::MarkNode { lhs, rhs } => (lhs.range().0, rhs.range().1),
            Self::MarkNodeShort(e) => (e.range().0 - 1, e.range().1),
            Self::MarkType { lhs, rhs } => (lhs.range().0, rhs.range().1),
            Self::MarkBranch { base, name, .. } => (base.range().0, name.range.1),
            Self::MustNot(e) => (e.range().0 - 1, e.range().1),
            Self::MustOne(e) => (e.range().0 - 1, e.range().1),
            Self::Maybe(e) => (e.range().0, e.range().1 + 1),
            Self::Many(e) => (e.range().0, e.range().1 + 1),
            Self::ManyNonNull(e) => (e.range().0, e.range().1 + 1),
        }
    }
    pub fn as_data(self) -> Option<Data> {
        match self {
            Expression::Data(e) => Some(e),
            _ => None,
        }
    }
    pub fn as_symbol_path(self) -> Option<SymbolPath> {
        match self.as_data() {
            Some(Data::Symbol(s)) => Some(s),
            _ => None,
        }
    }
    pub fn as_symbol(self) -> Option<Symbol> {
        let mut symbols = self.as_symbol_path().map(|e| e.symbol).unwrap_or_default();
        match symbols.len() {
            1 => Some(symbols.remove(0)),
            _ => None,
        }
    }
}

impl Data {
    pub fn range(&self) -> (usize, usize) {
        match self {
            Data::Symbol(e) => e.range,
            Data::Integer(e) => e.range,
            Data::String(e) => e.range,
            Data::Macro => (0, 0),
            Data::Regex => (0, 0),
        }
    }
}
