use std::ops::{Range, RangeInclusive};

use ucd_trie::Error;

use crate::CharacterSet;

pub const UNICODE_MAX: u32 = 0x110000;

#[derive(Debug, Clone)]
pub struct CharacterInsert {
    pub insert: Vec<Range<u32>>,
    pub error: Option<Error>,
}

impl CharacterSet {
    pub fn all() -> Self {
        Self { all: Box::new([true; UNICODE_MAX as usize]) }
    }

    pub fn nil() -> Self {
        Self { all: Box::new([false; UNICODE_MAX as usize]) }
    }
    pub fn include(&mut self, set: impl Into<CharacterInsert>) -> Result<(), Error> {
        self.try_insert(set, true)
    }
    pub fn exclude(&mut self, set: impl Into<CharacterInsert>) -> Result<(), Error> {
        self.try_insert(set, false)
    }
    fn try_insert(&mut self, set: impl Into<CharacterInsert>, result: bool) -> Result<(), Error> {
        let CharacterInsert { insert, error } = set.into();
        if let Some(e) = error {
            return Err(e);
        }
        self.insert(insert, result);
        Ok(())
    }
    fn insert(&mut self, set: Vec<Range<u32>>, result: bool) {
        for range in set {
            for cp in range {
                debug_assert!(cp <= UNICODE_MAX);
                self.all[cp as usize] = result;
            }
        }
    }
}

impl From<char> for CharacterInsert {
    fn from(char: char) -> Self {
        let end = char as u32 + 1;
        CharacterInsert { insert: vec![Range { start: char as u32, end }], error: None }
    }
}

impl From<Range<char>> for CharacterInsert {
    fn from(range: Range<char>) -> Self {
        let start = range.start as u32;
        let end = range.end as u32;
        Self::from((start, end))
    }
}

impl From<RangeInclusive<char>> for CharacterInsert {
    fn from(range: RangeInclusive<char>) -> Self {
        let start = (*range.start()) as u32;
        let end = (*range.end()) as u32 + 1;
        Self::from((start, end))
    }
}

impl From<u32> for CharacterInsert {
    fn from(char: u32) -> Self {
        CharacterInsert { insert: vec![Range { start: char, end: char + 1 }], error: None }
    }
}

impl From<(u32, u32)> for CharacterInsert {
    fn from(range: (u32, u32)) -> Self {
        let mut error = None;
        if range.1 > UNICODE_MAX {
            error = Some(Error::InvalidCodepoint(range.0))
        }
        if range.0 > UNICODE_MAX {
            error = Some(Error::InvalidCodepoint(range.1))
        }
        CharacterInsert { insert: vec![Range { start: range.0, end: range.1 }], error }
    }
}

impl From<Range<u32>> for CharacterInsert {
    fn from(range: Range<u32>) -> Self {
        let start = range.start;
        let end = range.end;
        Self::from((start, end))
    }
}

impl From<RangeInclusive<u32>> for CharacterInsert {
    fn from(range: RangeInclusive<u32>) -> Self {
        let start = *range.start();
        let end = range.end() + 1;
        Self::from((start, end))
    }
}
