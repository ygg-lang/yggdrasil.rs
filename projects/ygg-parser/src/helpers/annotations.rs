use super::*;

impl<'i> ClassStatementNode<'i> {
    pub fn annotations(&self) -> TakeAnnotations<'i> {
        // FIXME: AUTO TAG
        TakeAnnotations { auto_tag: self.op_remark().is_none(), macros: self.decorator_call(), modifiers: self.modifier_call() }
    }
}

impl<'i> UnionStatementNode<'i> {
    pub fn annotations(&self) -> TakeAnnotations<'i> {
        // FIXME: AUTO TAG
        TakeAnnotations { auto_tag: self.op_remark().is_none(), macros: self.decorator_call(), modifiers: self.modifier_call() }
    }
}

impl<'i> GroupStatementNode<'i> {
    pub fn annotations(&self) -> TakeAnnotations<'i> {
        TakeAnnotations { auto_tag: false, macros: self.decorator_call(), modifiers: self.modifier_call() }
    }
}

impl<'i> GroupPairNode<'i> {
    pub fn annotations(&self) -> TakeAnnotations<'i> {
        TakeAnnotations { auto_tag: false, macros: vec![], modifiers: vec![] }
    }
}

impl WithAnnotation for GrammarRule {
    fn with_annotation(mut self, extra: TakeAnnotations) -> Self {
        match extra.get_atomic() {
            Some(true) => self.atomic = GrammarAtomic::Atomic,
            Some(false) => self.atomic = GrammarAtomic::Combined,
            _ => {}
        };
        if let Some(s) = extra.get_hidden() {
            self.annotations.viewer.hidden = s
        };
        if let Some(s) = extra.get_railway() {
            self.annotations.viewer.railway = s
        };
        self.annotations.viewer.styles.extend(extra.get_styles());
        if let Some(s) = extra.get_auto_capture() {
            self.captures.auto = s
        };
        self
    }
}

impl<'i> TakeAnnotations<'i> {
    pub fn get_atomic(&self) -> Option<bool> {
        match self.get_hidden() {
            // ignored rule must atomic rule
            Some(true) => Some(true),
            _ => self.find_modifiers(&["atom", "atomic"], &["combine", "combined"]),
        }
    }
    pub fn get_hidden(&self) -> Option<bool> {
        self.find_modifiers(&["hide", "hidden"], &[])
    }
    pub fn get_entry(&self) -> Option<bool> {
        self.find_modifiers(&["entry"], &[])
    }

    pub fn get_railway(&self) -> Option<bool> {
        for body in self.find_functions("railway") {
            match body.expression().as_slice() {
                [first] => return first.clone().to_boolean(),
                _ => {}
            }
        }
        return None;
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
            for e in body.expression() {
                match e.to_identifier() {
                    Some(s) => out.push(s.get_str().to_string()),
                    None => {}
                }
            }
        }
        out
    }
}

impl<'i> TakeAnnotations<'i> {
    fn find_functions<'a>(&'i self, name: &'a str) -> impl Iterator<Item = CallBodyNode<'i>> + 'a
    where
        'i: 'a,
    {
        self.macros
            .iter()
            .filter(|v| v.decorator_name().identifier().get_str().eq_ignore_ascii_case(name))
            .map(move |v| v.call_body())
    }

    fn find_modifiers(&self, positive: &[&str], negative: &[&str]) -> Option<bool> {
        for m in self.modifiers.iter() {
            for accept in positive {
                if m.identifier().get_str().eq_ignore_ascii_case(accept) {
                    return Some(true);
                }
            }
            for reject in negative {
                if m.identifier().get_str().eq_ignore_ascii_case(reject) {
                    return Some(false);
                }
            }
        }
        None
    }
}
