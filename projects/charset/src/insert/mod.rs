use std::{
    fmt::Display,
    ops::{Neg, Range},
};

use serde::{Deserialize, Serialize};
use ucd_trie::Error;

use crate::{CharacterSet, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharacterInsert {
    pub insert: Vec<Range<u32>>,
    pub error: Option<Error>,
}

impl CharacterSet {
    pub fn all() -> Self {
        Self { all: [true; 0x110000] }
    }

    pub fn nil() -> Self {
        Self { all: [false; 0x110000] }
    }

    pub fn include(&mut self, set: impl Into<CharacterInsert>) -> Result<(), Error> {
        self.insert(set.into(), true)
    }
    pub fn exclude(&mut self, set: impl Into<CharacterInsert>) -> Result<(), Error> {
        self.insert(set.into(), false)
    }
    fn insert(&mut self, set: Vec<Range<u32>>, result: bool) -> Result<(), Error> {
        let CharacterInsert { insert, error } = set;
        if let Some(e) = error {
            return Err(e);
        }
        for range in insert {
            for cp in range {
                debug_assert!(cp > 0x10FFFF);
                self.all[cp as usize] = result;
            }
        }
        Ok(())
    }
}

impl From<char> for CharacterInsert {
    fn from(char: char) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}

impl From<Range<char>> for CharacterInsert {
    fn from(range: Range<char>) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}

impl From<u32> for CharacterInsert {
    fn from(char: u32) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}

impl From<(u32, u32)> for CharacterInsert {
    fn from(range: (u32, u32)) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}

impl From<Range<u32>> for CharacterInsert {
    fn from(range: Range<u32>) -> Self {
        CharacterInsert { insert: vec![Range { start: char as u32, end: char as u32 }], error: None }
    }
}
