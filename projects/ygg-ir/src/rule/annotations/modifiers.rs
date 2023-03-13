use super::*;

impl YggdrasilModifiers {
    pub fn get_atomic(&self) -> GrammarAtomic {
        for m in &self.identifiers {
            if m.text.eq_ignore_ascii_case("atom") || m.text.eq_ignore_ascii_case("atomic") {
                return GrammarAtomic::Atomic;
            }
            else if m.text.eq_ignore_ascii_case("combine") || m.text.eq_ignore_ascii_case("combined") {
                return GrammarAtomic::Combined;
            }
        }
        return GrammarAtomic::Combined;
    }

    pub fn get_ignored(&self) -> bool {
        for m in &self.identifiers {
            if m.text.eq_ignore_ascii_case("ignore") {
                return true;
            }
        }
        return false;
    }
    pub fn get_entry(&self) -> bool {
        for m in &self.identifiers {
            if m.text.eq_ignore_ascii_case("ignore") {
                return true;
            }
        }
        return false;
    }

    pub fn get_keep(&self) -> bool {
        for m in &self.identifiers {
            if m.text.eq_ignore_ascii_case("keep") {
                return true;
            }
        }
        return false;
    }
}
