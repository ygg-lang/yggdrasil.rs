use super::*;

impl Rule {
    fn is_inline_symbol(&self, rhs: &Rule) -> bool {
        rhs.name.data.starts_with("_")
    }

    fn inline(&mut self, _map: &GrammarInfo) {
        if self.already_inline {
            return;
        }
    }
}
