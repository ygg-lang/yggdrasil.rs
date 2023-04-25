use super::*;

impl YggdrasilModifiers {
    pub fn get_atomic(&self) -> Option<GrammarAtomic> {
        match self.find(&["atom", "atomic"], &["combine", "combined"])? {
            true => Some(GrammarAtomic::Atomic),
            false => Some(GrammarAtomic::Combined),
        }
    }

    pub fn get_ignored(&self) -> Option<bool> {
        self.find(&["ignore"], &[])
    }
    pub fn get_entry(&self) -> Option<bool> {
        self.find(&["entry"], &[])
    }
    pub fn get_keep(&self) -> Option<bool> {
        self.find(&["keep"], &[])
    }

    pub fn get_text(&self) -> Option<bool> {
        self.find(&["text"], &[])
    }
    fn find(&self, positive: &[&str], negative: &[&str]) -> Option<bool> {
        for m in &self.identifiers {
            for accept in positive {
                if m.text.eq_ignore_ascii_case(accept) {
                    return Some(true);
                }
            }
            for reject in negative {
                if m.text.eq_ignore_ascii_case(reject) {
                    return Some(false);
                }
            }
        }
        None
    }
}
