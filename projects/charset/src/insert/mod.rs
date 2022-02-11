use std::ops::Range;

use ucd_trie::Error;

use crate::CharacterSet;

#[derive(Debug, Clone)]
pub struct CharacterInsert {
    pub insert: Vec<Range<u32>>,
    pub error: Option<Error>,
}

impl CharacterSet {
    pub fn all() -> Self {
        Self { all: Box::new([true; 0x110000]) }
    }

    pub fn nil() -> Self {
        Self { all: Box::new([false; 0x110000]) }
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
            debug_assert!(range.end < 0x10FFFF);
            debug_assert!(range.start <= range.end);
            if range.start == range.end {
                self.all[range.start as usize] = result;
            }
            else {
                for cp in range {
                    self.all[cp as usize] = result;
                }
            }
        }
    }
}

impl From<char> for CharacterInsert {
    fn from(char: char) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}

impl From<Range<char>> for CharacterInsert {
    fn from(range: Range<char>) -> Self {
        CharacterInsert { insert: vec![Range { start: range.start as u32, end: range.end as u32 }], error: None }
    }
}

impl From<u32> for CharacterInsert {
    fn from(char: u32) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}

impl From<(u32, u32)> for CharacterInsert {
    fn from(range: (u32, u32)) -> Self {
        CharacterInsert { insert: vec![Range { start: range.0 as u32, end: range.1 as u32 }], error: None }
    }
}

impl From<Range<u32>> for CharacterInsert {
    fn from(range: Range<u32>) -> Self {
        CharacterInsert { insert: vec![Range { start: range.start as u32, end: range.end as u32 }], error: None }
    }
}
