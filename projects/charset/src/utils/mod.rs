use std::ops::Range;

use crate::{CharacterInsert, CharacterSet};

impl Default for CharacterSet {
    fn default() -> Self {
        Self { positive: true, set: Ti }
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
    pub fn exclude(&mut self, set: impl Into<CharacterInsert>) {}
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
