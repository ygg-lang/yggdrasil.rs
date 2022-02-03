use std::{
    fmt::{Display, Formatter},
    ops::{AddAssign, Range},
};

use crate::{CharacterInsert, CharacterSet};

impl Display for CharacterSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut w = &mut f.debug_set();
        for range in self.set.iter().chain(self.common.iter()) {
            if range.start == range.end {
                w = w.entry(&format!("{}", char::from_u32(range.start).unwrap()))
            }
            else {
                w = w.entry(&format!("{}..{}", char::from_u32(range.start).unwrap(), char::from_u32(range.end).unwrap()))
            }
        }
        w.finish()
    }
}

impl CharacterSet {
    pub fn include(&mut self, set: impl Into<CharacterInsert>) {
        #[cfg(debug_assertions)]
        {
            self.optimized = false;
        }
        let set = set.into();
        match set.fast {
            true => self.set.push(set.range),
            false => self.common.push(set.range),
        }
    }
    pub fn exclude(&mut self, set: impl Into<CharacterInsert>) {}
}

impl From<char> for CharacterInsert {
    fn from(char: char) -> Self {
        CharacterInsert { fast: false, range: Range { start: char as u32, end: char as u32 } }
    }
}

impl From<Range<char>> for CharacterInsert {
    fn from(range: Range<char>) -> Self {
        CharacterInsert { fast: false, range: Range { start: range.start as u32, end: range.end as u32 } }
    }
}

impl From<u32> for CharacterInsert {
    fn from(char: u32) -> Self {
        CharacterInsert { fast: false, range: Range { start: char, end: char } }
    }
}

impl From<(u32, u32)> for CharacterInsert {
    fn from(range: (u32, u32)) -> Self {
        CharacterInsert { fast: false, range: Range { start: range.0, end: range.1 } }
    }
}

impl From<Range<u32>> for CharacterInsert {
    fn from(range: Range<u32>) -> Self {
        CharacterInsert { fast: false, range: Range { start: range.start, end: range.end } }
    }
}

impl AddAssign<Self> for CharacterSet {
    fn add_assign(&mut self, rhs: Self) {
        #[cfg(debug_assertions)]
        {
            self.optimized = false;
        }
        self.common.extend(rhs.common.into_iter());
    }
}
