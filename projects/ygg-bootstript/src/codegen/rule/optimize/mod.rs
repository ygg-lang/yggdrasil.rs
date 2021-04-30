use super::*;
use crate::{codegen::GrammarState, manager::HintItems};
use lsp_types::{DocumentSymbol, SymbolKind};
use std::mem::transmute;

#[allow(deprecated)]
mod meta_info;

impl GrammarState {
    pub async fn optimize(&mut self) -> Result<HintItems> {
        let mut hint = HintItems::default();
        self.link_external().await?;
        hint += self.merge_regex()?;
        hint += self.inline()?;
        Ok(hint)
    }
    async fn link_external(&mut self) -> Result<()> {
        Ok(())
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
