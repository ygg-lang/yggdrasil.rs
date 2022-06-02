use std::ops::Range;

// pub(crate) mod cst_mode;
pub(crate) mod text_index;

pub mod state;

pub struct CapturedString {
    pub value: String,
    pub end: usize,
}

pub struct CapturedCharacter {
    pub value: char,
    pub start: usize,
}

impl CapturedString {
    pub fn new<S>(value: S, end: usize) -> Self
    where
        S: Into<String>,
    {
        Self { value: value.into(), end }
    }
    pub fn range(&self) -> Range<usize> {
        Range { start: self.end.saturating_sub(self.value.len()), end: self.end }
    }
}

impl CapturedCharacter {
    pub fn new(value: char, start: usize) -> Self {
        Self { value, start }
    }
    pub fn range(&self) -> Range<usize> {
        Range { start: self.start.saturating_sub(self.value.len_utf8()), end: self.start }
    }
}
