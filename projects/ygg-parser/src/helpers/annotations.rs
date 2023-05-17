use super::*;
use crate::bootstrap::{ClassBlockNode, ClassStatementNode};

impl ClassStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        TakeAnnotations { auto_tag: false, macros: &self.annotation_call, modifiers: &self.modifier_call }
    }
}

impl<'i> TakeAnnotations<'i> {
    pub fn get_atomic(&self) -> GrammarAtomic {
        self.modifiers.get_atomic().unwrap_or(GrammarAtomic::Combined)
    }
    pub fn get_ignored(&self) -> bool {
        self.modifiers.get_ignored().unwrap_or(false)
    }
    pub fn get_entry(&self) -> bool {
        self.modifiers.get_entry().unwrap_or(false)
    }

    pub fn get_keep(&self) -> bool {
        self.modifiers.get_keep().unwrap_or(false)
    }
    /// Whether to automatically mark all tags in the rule
    ///
    /// To ensure the highest priority, it needs to be called after with_annotation
    pub fn get_auto_capture(&self) -> Option<bool> {
        Some(self.auto_tag)
    }

    pub fn get_text_capture(&self) -> Option<bool> {
        self.modifiers.get_text()
    }
}

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
