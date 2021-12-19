use std::ops::Range;

use crate::CharacterInsert;

impl From<char> for CharacterInsert {
    fn from(char: char) -> Self {
        CharacterInsert {
            fast: false,
            range: Range { start: char as u32, end: char as u32 },
        }
    }
}

impl From<u32> for CharacterInsert {
    fn from(char: u32) -> Self {
        CharacterInsert {
            fast: false,
            range: Range { start: char, end: char },
        }
    }
}


impl From<(u32, u32)> for CharacterInsert {
    fn from(range: (u32, u32)) -> Self {
        CharacterInsert {
            fast: false,
            range: Range { start: range.0, end: range.1 },
        }
    }
}


impl From<Range<u32>> for CharacterInsert {
    fn from(range: Range<u32>) -> Self {
        CharacterInsert {
            fast: false,
            range: Range { start: range.start, end: range.end },
        }
    }
}
