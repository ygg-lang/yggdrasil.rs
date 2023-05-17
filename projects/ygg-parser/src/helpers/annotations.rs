use super::*;
use crate::bootstrap::{ClassStatementNode, GroupStatementNode, UnionStatementNode};

impl ClassStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        // FIXME: AUTO TAG
        TakeAnnotations { auto_tag: true, macros: &self.annotation_call, modifiers: &self.modifier_call }
    }
}

impl UnionStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        // FIXME: AUTO TAG
        TakeAnnotations { auto_tag: true, macros: &self.annotation_call, modifiers: &self.modifier_call }
    }
}

impl GroupStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        TakeAnnotations { auto_tag: false, macros: &self.annotation_call, modifiers: &self.modifier_call }
    }
}

impl<'i> TakeAnnotations<'i> {
    pub fn get_atomic(&self) -> Option<bool> {
        self.find_modifiers(&["atom", "atomic"], &["combine", "combined"])
    }
    pub fn get_ignored(&self) -> Option<bool> {
        self.find_modifiers(&["ignore"], &[])
    }
    pub fn get_entry(&self) -> Option<bool> {
        self.find_modifiers(&["entry"], &[])
    }

    pub fn get_keep(&self) -> Option<bool> {
        self.find_modifiers(&["keep"], &[])
    }
    /// Whether to automatically mark all tags in the rule
    ///
    /// To ensure the highest priority, it needs to be called after with_annotation
    pub fn get_auto_capture(&self) -> Option<bool> {
        Some(self.auto_tag)
    }
    pub fn get_text_capture(&self) -> Option<bool> {
        self.find_modifiers(&["text"], &[])
    }
}

impl<'i> TakeAnnotations<'i> {
    fn find_modifiers(&self, positive: &[&str], negative: &[&str]) -> Option<bool> {
        for m in self.modifiers {
            for accept in positive {
                if m.identifier.text.eq_ignore_ascii_case(accept) {
                    return Some(true);
                }
            }
            for reject in negative {
                if m.identifier.text.eq_ignore_ascii_case(reject) {
                    return Some(false);
                }
            }
        }
        None
    }
}
