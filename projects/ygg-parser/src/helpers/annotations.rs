use crate::bootstrap::{CallBodyNode, ClassStatementNode, GroupPairNode, GroupStatementNode, UnionStatementNode};

use super::*;

impl ClassStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        // FIXME: AUTO TAG
        TakeAnnotations { auto_tag: self.op_remark.is_none(), macros: &self.decorator_call, modifiers: &self.modifier_call }
    }
}

impl UnionStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        // FIXME: AUTO TAG
        TakeAnnotations { auto_tag: self.op_remark.is_none(), macros: &self.decorator_call, modifiers: &self.modifier_call }
    }
}

impl GroupStatementNode {
    pub fn annotations(&self) -> TakeAnnotations {
        TakeAnnotations { auto_tag: false, macros: &self.decorator_call, modifiers: &self.modifier_call }
    }
}

impl GroupPairNode {
    pub fn annotations(&self) -> TakeAnnotations {
        TakeAnnotations { auto_tag: false, macros: &[], modifiers: &[] }
    }
}

impl<'i> TakeAnnotations<'i> {
    pub fn get_atomic(&self) -> Option<bool> {
        match self.get_ignored() {
            // ignored rule must atomic rule
            Some(true) => Some(true),
            _ => self.find_modifiers(&["atom", "atomic"], &["combine", "combined"]),
        }
    }
    pub fn get_ignored(&self) -> Option<bool> {
        self.find_modifiers(&["ignore", "ignored"], &[])
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
    pub fn get_styles(&self) -> Vec<String> {
        let mut out = vec![];
        for body in self.find_functions("style") {
            for e in &body.expression {
                match e.as_identifier() {
                    Some(s) => out.push(s.text.clone()),
                    None => {}
                }
            }
        }
        out
    }
}

impl<'i> TakeAnnotations<'i> {
    fn find_functions<'a>(&'i self, name: &'a str) -> impl Iterator<Item = &'i CallBodyNode> + 'a
    where
        'i: 'a,
    {
        self.macros.iter().filter(|v| v.decorator_name.identifier.text.eq_ignore_ascii_case(name)).map(move |v| &v.call_body)
    }

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
