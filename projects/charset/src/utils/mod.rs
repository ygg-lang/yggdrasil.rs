use core::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Range;

use crate::{CharacterInsert, CharacterSet};

impl Default for CharacterSet {
    fn default() -> Self {
        Self {
            #[cfg(debug_assertions)]
            optimized: true,
            fast: vec![],
            common: vec![],
        }
    }
}

impl CharacterSet {
    /// Count how many characters are in this set
    pub fn count(&self) -> usize {
        debug_assert!(self.optimized);
        let mut all = 0;
        for rule in &self.common {
            all += rule.end - rule.start + 1
        }
        return all as usize;
    }
    /// Determines whether the set contains the given character
    pub fn contains(&self, c: char) -> bool {
        debug_assert!(self.optimized);
        let c = c as u32;
        for rule in &self.fast {
            if rule.start <= c && c <= rule.end {
                return true;
            }
        }
        self.bsearch_range_table(&self.common, c)
    }

    // Because ASCII ranges are at the start of the tables, a search for an
    // ASCII char will involve more `Greater` results (i.e. the `(lo,hi)`
    // table entry is greater than `c`) than `Less` results. And given that
    // ASCII chars are so common, it makes sense to favor them. Therefore,
    // the `Greater` case is tested for before the `Less` case.
    fn bsearch_range_table(&self, r: &[Range<u32>], c: u32) -> bool {
        r.binary_search_by(|range| {
            if range.start > c {
                Greater
            }
            else if range.end < c {
                Less
            }
            else {
                Equal
            }
        })
        .is_ok()
    }
}

impl CharacterSet {
    pub fn insert(&mut self, set: impl Into<CharacterInsert>) {
        #[cfg(debug_assertions)]
        {
            self.optimized = false;
        }
        let set = set.into();
        match set.fast {
            true => self.fast.push(set.range),
            false => self.common.push(set.range),
        }
    }
    pub fn optimize(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.optimized = true;
        }
        let mut new: Vec<Range<u32>> = vec![];
        self.common.sort_by_key(|r| r.start);
        for Range { start, end } in &self.common {
            match new.last_mut() {
                Some(last) => {
                    if last.end >= start.saturating_sub(1) {
                        last.end = last.end.max(*end)
                    }
                    else {
                        new.push(Range { start: *start, end: *end })
                    }
                }
                None => new.push(Range { start: *start, end: *end }),
            }
        }
        new.truncate(new.len());
        self.common = new;
    }
}
