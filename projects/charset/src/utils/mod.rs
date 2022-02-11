use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use ucd_trie::TrieSetOwned;

mod arithmetic;
mod save;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct CharacterSet {
    pub(crate) all: Box<[bool; 0x110000]>,
}

impl Default for CharacterSet {
    fn default() -> Self {
        Self::nil()
    }
}

impl Debug for CharacterSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CharacterSet({}) ", self.count())?;
        let mut w = &mut f.debug_set();
        for range in self.to_ranges() {
            if range.start == range.end {
                w = w.entry(&format!("{}", range.start as u32))
            }
            else {
                w = w.entry(&format!("{}..{}", range.start as u32, range.end as u32))
            }
        }
        w.finish()
    }
}

impl Display for CharacterSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CharacterSet({}) ", self.count())?;
        let mut w = &mut f.debug_set();
        for range in self.to_ranges() {
            if range.start == range.end {
                w = w.entry(&format!("{}", range.start))
            }
            else {
                w = w.entry(&format!("{}..{}", range.start, range.end))
            }
        }
        w.finish()
    }
}

impl CharacterSet {
    /// Count how many characters are in this set
    pub fn count(&self) -> usize {
        self.all.iter().filter(|f| **f == true).count()
    }
    /// Determines whether the set contains the given character
    pub fn compress(&self) -> TrieSetOwned {
        let set = TrieSetOwned::from_codepoints(self.codepoints());
        #[cfg(debug_assertions)]
        {
            set.unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            unsafe { set.unwrap_unchecked() }
        }
    }
    pub fn contains(&self, c: char) -> bool {
        self.compress().contains_char(c)
    }
    fn codepoints(&self) -> Vec<u32> {
        let mut codepoints = vec![];
        let mut this_cp: u32 = 0;
        for contains in self.all.iter() {
            if *contains {
                codepoints.push(this_cp)
            }
            this_cp += 1;
        }
        return codepoints;
    }

    pub fn to_ranges(&self) -> Vec<Range<char>> {
        let mut ranges = vec![];
        for cp in self.codepoints() {
            range_add(&mut ranges, cp);
        }
        ranges.into_iter().map(|(min, max)| range_u2c(min, max)).collect()
    }
}

#[track_caller]
pub(crate) fn range_u2c(start: u32, end: u32) -> Range<char> {
    #[cfg(debug_assertions)]
    {
        let start = char::from_u32(start).unwrap();
        let end = char::from_u32(end).unwrap();
        Range { start, end }
    }
    #[cfg(not(debug_assertions))]
    {
        let start = char::from_u32_unchecked(start);
        let end = char::from_u32_unchecked(end);
        Range { start, end }
    }
}

/// https://github.com/BurntSushi/ucd-generate/blob/07c11775dbc8e659e5e9485284f74fe7429ead6c/src/util.rs#L206
fn range_add(ranges: &mut Vec<(u32, u32)>, codepoint: u32) {
    if let Some(&mut (_, ref mut end)) = ranges.last_mut() {
        assert!(*end < codepoint);
        if codepoint == *end + 1 {
            *end = codepoint;
            return;
        }
    }
    ranges.push((codepoint, codepoint));
}
