mod arth;

use std::ops::Range;

use ucd_trie::{Error, TrieSetOwned};

use crate::CharacterInsert;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CharacterSet {
    pub(crate) all: [bool; 0x110000],
}

impl Default for CharacterSet {
    fn default() -> Self {
        Self::nil()
    }
}

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
    /// Count how many characters are in this set
    pub fn count(&self) -> usize {
        for ucd_trie in self.set {}
    }
    /// Determines whether the set contains the given character
    pub fn contains(&self, c: char) -> bool {
        self.set.contains_char(c)
    }
}

impl CharacterSet {
    pub fn from_codepoints<I, C>(codepoints: I) -> Result<TrieSetOwned>
    where
        I: IntoIterator<Item = C>,
        C: Borrow<u32>,
    {
        let mut all = vec![false; 0x110000];
        for cp in codepoints {
            let cp = *cp.borrow();
            if cp > 0x10FFFF {
                return Err(Error::InvalidCodepoint(cp));
            }
            all[cp as usize] = true;
        }
        TrieSetOwned::new(&all)
    }
    pub fn to_ranges(&self) -> Vec<Range<char>> {
        let mut codepoints: Vec<u32> = self.set.into_iter().collect();
        codepoints.sort();
        codepoints.dedup();

        let mut ranges = vec![];
        for cp in codepoints {
            range_add(&mut ranges, cp);
        }
        ranges.into_iter().map(|(min, max)| Range { start: min as char, end: max as char }).collect()
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
