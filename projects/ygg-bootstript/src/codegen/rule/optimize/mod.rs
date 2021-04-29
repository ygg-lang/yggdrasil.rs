use super::*;
use crate::{codegen::GrammarState, manager::HintItems};
use lsp_types::{DocumentSymbol, SymbolKind};
use std::mem::transmute;

#[allow(deprecated)]
mod meta_info;

impl GrammarState {
    pub fn optimize(&mut self) -> Result<HintItems> {
        let mut hint = HintItems::default();
        hint += self.merge_regex()?;
        hint += self.inline()?;
        Ok(hint)
    }
    fn inline(&mut self) -> Result<HintItems> {
        Ok(HintItems::default())
    }
    fn merge_regex(&mut self) -> Result<HintItems> {
        Ok(HintItems::default())
    }
}

impl YGGRule {
    fn inline(&mut self, _map: &GrammarState) {}
    fn merge_regex(&mut self) {}
}
