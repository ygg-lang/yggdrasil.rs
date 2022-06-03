use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};

// pub(crate) mod cst_mode;
pub(crate) mod text_index;

pub mod state;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CapturedString {
    pub value: &'static str,
    pub start: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CapturedCharacter {
    pub value: char,
    pub start: usize,
}

impl Debug for CapturedString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("String").field("value", &self.value).field("range", &self.range()).finish()
    }
}

impl Display for CapturedString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value)
    }
}

impl Debug for CapturedCharacter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Character").field("value", &self.value).field("range", &self.range()).finish()
    }
}

impl Display for CapturedCharacter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.value)
    }
}

impl CapturedString {
    pub fn new(value: &'static str, end: usize) -> Self {
        Self { value, start: end }
    }
    pub fn range(&self) -> Range<usize> {
        Range { start: self.start, end: self.start + self.value.len() }
    }
}

impl CapturedCharacter {
    pub fn new(value: char, start: usize) -> Self {
        Self { value, start }
    }
    pub fn range(&self) -> Range<usize> {
        Range { start: self.start, end: self.start + self.value.len_utf8() }
    }
}
