use super::*;
use crate::YggdrasilLanguage;
use alloc::borrow::Cow;

impl<Language, Input> ConcreteNode<Language, Input> {
    // SAFETY: Theoretically all ids generated by lexer are legal
    pub(crate) unsafe fn get_data(&self) -> &ConcreteNodeData {
        if cfg!(debug_assertions) {
            self.shared.arena.get(self.id).expect("missing node")
        }
        else {
            self.shared.arena.get_unchecked(self.id)
        }
    }
}

impl<Language, Input> ConcreteNode<Language, Input>
where
    Language: YggdrasilLanguage,
    Input: TextStream,
{
    pub fn get_parent(&self) -> Option<ConcreteNode<Language, Input>> {
        let parent = unsafe { self.get_data().parent? };
        Some(Self { shared: self.shared.clone(), id: parent })
    }
    pub fn get_next(&self) -> Option<ConcreteNode<Language, Input>> {
        let next = unsafe { self.get_data().next? };
        Some(Self { shared: self.shared.clone(), id: next })
    }
    pub fn get_back(&self) -> Option<ConcreteNode<Language, Input>> {
        let back = unsafe { self.get_data().prev? };
        Some(Self { shared: self.shared.clone(), id: back })
    }

    pub fn get_siblings(&self) -> ConcreteNodeIterator<Language, Input> {
        let data = unsafe { self.get_data() };
        ConcreteNodeIterator {
            skip_hidden: false,
            shared: self.shared.clone(),
            prev: Some(data.brother.start),
            next: Some(data.brother.end),
        }
    }
    pub fn get_children(&self) -> ConcreteNodeIterator<Language, Input> {
        let data = unsafe { self.get_data() };
        ConcreteNodeIterator {
            skip_hidden: false,
            shared: self.shared.clone(),
            prev: data.children.as_ref().map(|s| s.start),
            next: data.children.as_ref().map(|s| s.end),
        }
    }
    pub fn get_language(&self) -> &Language {
        &self.shared.language
    }
    pub fn get_rule(&self) -> Language::Rule {
        let rule_id = unsafe { self.get_data().rule };

        if cfg!(debug_assertions) {
            match rule_id.try_into() {
                Ok(rule) => rule,
                Err(_) => panic!("invalid rule id: {}", rule_id),
            }
        }
        else {
            // SAFETY: Theoretically all ids generated by lexer are legal
            unsafe { rule_id.try_into().unwrap_unchecked() }
        }
    }
    pub fn get_input(&self) -> &Input {
        &self.shared.input
    }
    pub fn get_span(&self) -> Range<usize> {
        unsafe { self.get_data().span.clone() }
    }
    pub fn get_text(&self) -> Cow<str> {
        let span = unsafe { &self.get_data().span };
        let text = self.shared.input.text(span);
        if cfg!(debug_assertions) {
            text.expect("invalid text range")
        }
        else {
            // SAFETY: Theoretically all span generated by lexer are legal
            unsafe { text.unwrap_unchecked() }
        }
    }
}
