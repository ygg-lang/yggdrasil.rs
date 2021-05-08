use super::*;

impl YGGRule {
    fn is_inline_symbol(&self, rhs: &YGGRule) -> bool {
        rhs.name.data.starts_with("_")
    }

    fn inline(&mut self, _map: &GrammarState) {
        if self.already_inline {
            return;
        }
    }
}
