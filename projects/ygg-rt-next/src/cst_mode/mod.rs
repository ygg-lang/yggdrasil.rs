use std::{borrow::Cow, marker::PhantomData, ops::Range, rc::Rc};

mod iters;

pub trait YggdrasilRule: From<u32> {}

pub trait YggdrasilLanguage {
    type Rule: YggdrasilRule;
}

pub trait InputStream {
    fn get_file_name(&self) -> Option<&str> {
        None
    }
    fn get_text(&self, span: &Range<usize>) -> Cow<str>;
}

impl<'i> InputStream for &'i str {
    fn get_text(&self, span: &Range<usize>) -> Cow<'i, str> {
        let text = if cfg!(debug_assertions) {
            self.get(span.start..span.end).expect("invalid span")
        }
        else {
            unsafe { self.get_unchecked(span.start..span.end) }
        };
        Cow::Borrowed(text)
    }
}

pub struct ConcreteTree<Language, Input> {
    input: Input,
    language: Language,
    arena: Vec<ConcreteNodeData>,
}

pub struct ConcreteNodeData {
    this: usize,
    parent: Option<usize>,
    prev: Option<usize>,
    next: Option<usize>,
    brother: Range<usize>,
    children: Option<Range<usize>>,
    rule: u32,
    span: Range<usize>,
}

pub struct ConcreteNode<Language, Input> {
    shared: Rc<ConcreteTree<Language, Input>>,
    id: usize,
}

pub struct ConcreteNodeIterator<Language, Input> {
    shared: Rc<ConcreteTree<Language, Input>>,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<Language, Input> ConcreteNode<Language, Input>
where
    Language: YggdrasilLanguage,
    Input: InputStream,
{
    fn get_data(&self) -> Option<&ConcreteNodeData> {
        self.shared.arena.get(self.id)
    }
    pub fn get_parent(&self) -> Option<ConcreteNode<Language, Input>> {
        Some(Self { shared: self.shared.clone(), id: self.get_data()?.parent? })
    }
    pub fn get_next(&self) -> Option<ConcreteNode<Language, Input>> {
        Some(Self { shared: self.shared.clone(), id: self.get_data()?.next? })
    }
    pub fn get_back(&self) -> Option<ConcreteNode<Language, Input>> {
        Some(Self { shared: self.shared.clone(), id: self.get_data()?.prev? })
    }

    pub fn get_siblings(&self) -> ConcreteNodeIterator<Language, Input>

    pub fn get_rule(&self) -> Language::Rule {
        self.get_data()
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }
    pub fn get_language(&self) -> &Language {
        &self.language
    }
    pub fn get_text(&self) -> Cow<str> {
        self.input.get_text(&self.span)
    }
}
