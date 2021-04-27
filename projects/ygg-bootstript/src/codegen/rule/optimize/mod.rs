use super::*;

impl GrammarState {
    pub fn optimize(&mut self) -> Result<HintItems> {
        let mut hint = HintItems::default();
        hint += self.merge_regex()?;
        hint += self.inline()?;
        Ok(hint)
    }
    fn inline(&mut self) -> Result<HintItems> {
        unimplemented!()
    }
    fn merge_regex(&mut self) -> Result<HintItems> {
        unimplemented!()
    }
}

impl YGGRule {
    fn inline(&mut self, _map: &GrammarState) {}
    fn merge_regex(&mut self) {}
}
