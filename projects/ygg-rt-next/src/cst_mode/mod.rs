use std::{borrow::Cow, ops::Range};

pub trait YggdrasilRule: From<u32> {}

pub trait YggdrasilLanguage {
    type Rule: YggdrasilRule;
}

pub trait InputStream {
    fn get_file_name(&self) -> &str;
    fn get_text(&self, span: &Range<usize>) -> Cow<str>;
}

impl<'i> InputStream for &'i str {
    fn get_file_name(&self) -> &str {
        todo!()
    }

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

pub struct ConcreteNode<Language, Input> {
    rule: u32,
    language: Language,
    input: Input,
    span: Range<usize>,
}

impl<Language, Input> ConcreteNode<Language, Input>
where
    Language: YggdrasilLanguage,
    Input: InputStream,
{
    pub fn get_rule(&self) -> Language::Rule {
        <Language as YggdrasilLanguage>::Rule::from(self.rule)
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
