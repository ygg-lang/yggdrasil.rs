// Package charset provides data types and functions for managing sets of
// characters.
use std::fmt;

const LOG2_WORD_SIZE: u8 = 6;
const WORD_SIZE: u64 = 64;

// A Set represents a set of chars.
pub struct Set {
    // Bits is the bit array for indicating which chars are in the set.
    // We have 256 bits because a char can have 256 different values.
    bits: [u64; 4],
}

// A SmallSet is the same as a Set but can only represent 128 possible chars.
// This is an optimization, since in the common case, only ASCII bytes are
// used which are <128. The full Set is only necessary when unicode control
// characters must be matched.
pub struct SmallSet {
    bits: [u64; 2],
}

impl SmallSet {
    // Size returns the number of chars matched by this Set.
    pub fn size(&self) -> usize {
        self.bits[0].count_ones() as usize + self.bits[1].count_ones() as usize
    }

    // Has checks if a charset accepts a character.
    // Pointer receiver is for performance.
    pub fn has(&self, r: u8) -> bool {
        let index = r >> LOG2_WORD_SIZE;
        let bit = 1u64 << (r as u64 & (WORD_SIZE - 1));
        self.bits[index as usize] & bit != 0
    }
}

impl Set {
    // IsSmall returns true if this set can be converted to a small set. In other
    // words, if this set only matches bytes <128.
    pub fn is_small(&self) -> bool {
        self.bits[2] == 0 && self.bits[3] == 0
    }

    // SmallSet converts this Set to a SmallSet.
    pub fn small_set(&self) -> SmallSet {
        SmallSet { bits: [self.bits[0], self.bits[1]] }
    }
}

// New returns a charset which accepts all chars in 'chars'. Note
// that all chars must be valid ASCII characters (<128).
pub fn new(chars: &[u8]) -> Set {
    let mut set = Set { bits: [0; 4] };

    for &r in chars {
        match r {
            0..=63 => set.bits[0] |= 1u64 << r,
            64..=127 => set.bits[1] |= 1u64 << (r - 64),
            128..=191 => set.bits[2] |= 1u64 << (r - 128),
            _ => set.bits[3] |= 1u64 << (r - 192),
        }
    }

    set
}

// CharsetRange returns a charset matching all characters between `low` and
// `high` inclusive.
pub fn range(low: u8, high: u8) -> Set {
    let mut set = Set { bits: [0; 4] };

    for c in low..=high {
        match c {
            0..=63 => set.bits[0] |= 1u64 << c,
            64..=127 => set.bits[1] |= 1u64 << (c - 64),
            128..=191 => set.bits[2] |= 1u64 << (c - 128),
            _ => set.bits[3] |= 1u64 << (c - 192),
        }
    }

    set
}

impl Set {
    // Complement returns a charset that matches all characters except for those
    // matched by `c`.
    pub fn complement(&self) -> Set {
        Set { bits: [!self.bits[0], !self.bits[1], !self.bits[2], !self.bits[3]] }
    }

    // Add combines the characters two charsets match together.
    pub fn add(&self, other: Set) -> Set {
        Set {
            bits: [
                self.bits[0] | other.bits[0],
                self.bits[1] | other.bits[1],
                self.bits[2] | other.bits[2],
                self.bits[3] | other.bits[3],
            ],
        }
    }

    // Sub removes from 'c' any characters in 'other'.
    pub fn sub(&self, other: Set) -> Set {
        Set {
            bits: [
                !other.bits[0] & self.bits[0],
                !other.bits[1] & self.bits[1],
                !other.bits[2] & self.bits[2],
                !other.bits[3] & self.bits[3],
            ],
        }
    }

    // Size returns the number of chars matched by this Set.
    pub fn size(&self) -> usize {
        self.bits[0].count_ones() as usize
            + self.bits[1].count_ones() as usize
            + self.bits[2].count_ones() as usize
            + self.bits[3].count_ones() as usize
    }

    // Has checks if a charset accepts a character.
    // Pointer receiver is for performance.
    pub fn has(&self, r: u8) -> bool {
        let index = r >> LOG2_WORD_SIZE;
        let bit = 1u64 << (r as u64 & (WORD_SIZE - 1));
        self.bits[index as usize] & bit != 0
    }
}

impl fmt::Display for Set {
    // String returns the string representation of the charset.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut in_range = false;

        for b in 0..=255 {
            if self.has(b) && b == 255 {
                s += &format!("{:?}", b as char);
            }
            else if self.has(b) && !in_range {
                in_range = true;
                if self.has(b + 1) {
                    s += &format!("{:?}..", b as char);
                }
            }
            else if !self.has(b) && in_range {
                in_range = false;
                s += &format!("{:?},", (b - 1) as char);
            }
        }

        if !s.is_empty() && s.chars().last() == Some(',') {
            s.pop();
        }

        write!(f, "{{{}}}", s)
    }
}
