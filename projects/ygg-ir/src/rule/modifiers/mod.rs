use crate::rule::{GrammarAtomic, YggdrasilIdentifier};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilAnnotations {
    pub macros: Vec<usize>,
    pub modifiers: Vec<YggdrasilIdentifier>,
}

impl YggdrasilAnnotations {
    pub fn get_atomic(&self) -> GrammarAtomic {
        for m in &self.modifiers {
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
        for m in &self.modifiers {
            if m.text.eq_ignore_ascii_case("ignore") {
                return true;
            }
        }
        return false;
    }
    pub fn get_entry(&self) -> bool {
        for m in &self.modifiers {
            if m.text.eq_ignore_ascii_case("ignore") {
                return true;
            }
        }
        return false;
    }

    pub fn get_keep(&self) -> bool {
        for m in &self.modifiers {
            if m.text.eq_ignore_ascii_case("keep") {
                return true;
            }
        }
        return false;
    }
}
