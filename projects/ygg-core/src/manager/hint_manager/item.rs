use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct HintItems {
    pub diagnostic: Vec<Diagnostic>,
    pub code_lens: Vec<CodeLens>,
    pub document_symbol: Vec<DocumentSymbol>,
}

impl Default for HintItems {
    fn default() -> Self {
        Self { diagnostic: vec![], code_lens: vec![], document_symbol: vec![] }
    }
}

#[rustfmt::skip]
impl Add<Self> for HintItems {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            diagnostic: self.diagnostic.into_iter().chain(rhs.diagnostic.into_iter()).collect(),
            code_lens: self.code_lens.into_iter().chain(rhs.code_lens.into_iter()).collect(),
            document_symbol: self.document_symbol.into_iter().chain(rhs.document_symbol.into_iter()).collect(),
        }
    }
}

impl AddAssign<Self> for HintItems {
    fn add_assign(&mut self, rhs: Self) {
        self.diagnostic.extend(rhs.diagnostic);
        self.code_lens.extend(rhs.code_lens);
        self.document_symbol.extend(rhs.document_symbol);
    }
}
