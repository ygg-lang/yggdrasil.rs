use std::ops::{Range, RangeInclusive};

use ucd_trie::Error;

use crate::CharacterSet;

pub const UNICODE_MAX: u32 = 0x110000;

#[derive(Debug, Clone)]
pub struct InsertAction {
    pub insert: Vec<RangeInclusive<u32>>,
    pub error: Option<Error>,
}

impl CharacterSet {
    pub fn all() -> Self {
        Self { all: Box::new([true; UNICODE_MAX as usize]) }
    }
    pub fn nil() -> Self {
        Self { all: Box::new([false; UNICODE_MAX as usize]) }
    }
    pub fn include(&mut self, set: impl Into<InsertAction>) -> Result<(), Error> {
        self.try_insert(set, true)
    }
    pub fn exclude(&mut self, set: impl Into<InsertAction>) -> Result<(), Error> {
        self.try_insert(set, false)
    }
    fn try_insert(&mut self, set: impl Into<InsertAction>, result: bool) -> Result<(), Error> {
        let InsertAction { insert, error } = set.into();
        if let Some(e) = error {
            return Err(e);
        }
        self.insert(insert, result);
        Ok(())
    }
    fn insert(&mut self, set: Vec<RangeInclusive<u32>>, result: bool) {
        for range in set {
            for cp in range {
                debug_assert!(cp <= UNICODE_MAX);
                self.all[cp as usize] = result;
            }
        }
    }
}

impl From<char> for InsertAction {
    fn from(char: char) -> Self {
        InsertAction::from(RangeInclusive::new(char, char))
    }
}

impl From<RangeInclusive<char>> for InsertAction {
    fn from(range: RangeInclusive<char>) -> Self {
        let start = *range.start() as u32;
        let end = *range.end() as u32;
        InsertAction { insert: vec![RangeInclusive::new(start, end)], error: None }
    }
}

impl From<Range<char>> for InsertAction {
    fn from(range: Range<char>) -> Self {
        InsertAction::from(RangeInclusive::new(range.start as u32, range.end as u32 + 1))
    }
}

impl From<(char, char)> for InsertAction {
    fn from(range: (char, char)) -> Self {
        InsertAction::from(RangeInclusive::new(range.0, range.1))
    }
}

impl From<u32> for InsertAction {
    fn from(char: u32) -> Self {
        InsertAction::from(RangeInclusive::new(char, char))
    }
}

impl From<RangeInclusive<u32>> for InsertAction {
    fn from(range: RangeInclusive<u32>) -> Self {
        let mut error = None;
        let start = *range.start();
        let end = *range.end();
        if start > UNICODE_MAX {
            error = Some(Error::InvalidCodepoint(start))
        }
        if end > UNICODE_MAX {
            error = Some(Error::InvalidCodepoint(end))
        }
        InsertAction { insert: vec![RangeInclusive::new(start, end)], error }
    }
}
impl From<Range<u32>> for InsertAction {
    fn from(range: Range<u32>) -> Self {
        InsertAction::from(RangeInclusive::new(range.start, range.end + 1))
    }
}

impl From<(u32, u32)> for InsertAction {
    fn from(range: (u32, u32)) -> Self {
        InsertAction::from(RangeInclusive::new(range.0, range.1))
    }
}
