use super::*;
use yggdrasil_ir::rule::GrammarRuleAttributes;

pub struct TakeAnnotations<'i> {
    pub auto_tag: bool,
    pub macros: Vec<DecoratorCallNode<'i>>,
    pub modifiers: Vec<ModifierCallNode<'i>>,
}

impl<'i> AstBuilder<'i> for TakeAnnotations<'i> {
    type Output = GrammarRuleAttributes;

    fn build(&self, _: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        let mut out = GrammarRuleAttributes::default();
        match self.get_atomic() {
            Some(true) => out.atomic = GrammarAtomic::Atomic,
            Some(false) => out.atomic = GrammarAtomic::Combined,
            _ => {}
        };
        if let Some(s) = self.get_hidden() {
            out.viewer.hidden = s
        };
        if let Some(s) = self.get_railway() {
            out.viewer.railway = s
        };
        out.viewer.styles.extend(self.get_styles());
        if let Some(s) = self.get_auto_capture() {
            out.captures = s
        };
        Ok(out)
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
