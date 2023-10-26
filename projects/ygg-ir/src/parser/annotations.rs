use super::*;

impl GrammarRule {
    pub(super) fn with_annotation(mut self, extra: TakeAnnotations) -> Self {
        match extra.get_atomic() {
            Some(true) => self.atomic = GrammarAtomic::Atomic,
            Some(false) => self.atomic = GrammarAtomic::Combined,
            _ => {}
        };
        if let Some(s) = extra.get_hidden() {
            self.hidden = s
        };
        if let Some(s) = extra.get_keep() {
            self.view = s
        };
        if let Some(s) = extra.get_entry() {
            self.entry = s
        };
        if let Some(s) = extra.get_auto_capture() {
            self.captures.auto = s
        };
        if let Some(s) = extra.get_text_capture() {
            self.captures.text = s
        }
        self
    }
}
