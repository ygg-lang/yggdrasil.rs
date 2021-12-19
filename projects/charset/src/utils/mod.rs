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
    fn contains(&self, c: char) -> bool {
        debug_assert!(self.optimized);
        for rule in &self.fast {
            if rule.contains(&c) {
                return true
            }
        }
        self.bsearch_range_table(&self.common, c)
    }

    // Because ASCII ranges are at the start of the tables, a search for an
    // ASCII char will involve more `Greater` results (i.e. the `(lo,hi)`
    // table entry is greater than `c`) than `Less` results. And given that
    // ASCII chars are so common, it makes sense to favor them. Therefore,
    // the `Greater` case is tested for before the `Less` case.
    fn bsearch_range_table(&self, r: &[Range<char>], c: char) -> bool {
        r.binary_search_by(|&range| {
            if range.start > c {
                Greater
            } else if range.end < c {
                Less
            } else {
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
            true => { self.fast.push(set.range) }
            false => { self.common.push(set.range) }
        }
    }
    pub fn optimize(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.optimized = true;
        }
        // a = [(7, 10), (11, 13), (11, 15), (14, 20), (23, 39)]
        // b = []
        // for begin,end in sorted(a):
        //     if b and b[-1][1] >= begin - 1:
        //         b[-1][1] = max(b[-1][1], end)
        //     else:
        //         b.append([begin, end])

        let mut new: Vec<Range<char>> = vec![];
        self.common.sort_by_key(|r| r.start);
        for Range { start, end } in &self.common {
            match new.last_mut() {
                Some(last) => {
                    if last.start >= start - 1 {
                        last.start = last.start.max(*end)
                    } else {
                        new.push(Range { start: *start, end: *end })
                    }
                }
                None => break
            }
        };
        new.truncate(new.len());
        self.common = new;
    }
}

#[test]
fn test() {
    let mut set = CharacterSet::default();
    set.insert('a'..'z');
    set.insert('A'..'Z');
    set.insert('0'..'9');
    println!("{:#?}", set)
}