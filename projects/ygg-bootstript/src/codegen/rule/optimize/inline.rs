use super::*;

impl YGGRule {
    fn is_inline_symbol(&self, rhs: &YGGRule) {}

    fn inline(&mut self, _map: &GrammarState) {
        if self.already_inline {
            return;
        }
    }
}
