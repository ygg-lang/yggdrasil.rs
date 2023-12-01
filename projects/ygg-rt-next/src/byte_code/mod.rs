use alloc::{string::String, vec::Vec};
use core::ops::Range;
use daachorse::CharwiseDoubleArrayAhoCorasick;

pub struct Bytecode {
    indentation: bool,
    patterns: Vec<Instruction>,
    strings: Vec<StringPattern>,
}

pub enum Instruction {
    Any,
    Character {
        /// A unicode character
        unicode: char,
    },
    Range {
        /// Weather this is a negative range
        negative: bool,
        /// The character range
        span: Range<char>,
    },
    MatchString {
        index: u32,
    },
    MatchRule {
        /// The rule id of the language
        rule: u32,
        /// The index of start rule
        span: Range<u32>,
    },
    MatchTag {
        /// The tag id of the language
        tag: u32,
        /// The index of start tag
        span: Range<u32>,
    },
    LookAhead {
        negative: bool,
        /// The index of start tag
        jump: u32,
        /// The length of the pattern
        length: u32,
    },
    StartOfStream,
    EndOfStream,
}

impl Bytecode {
    pub fn sequence(&self, span: &Range<u32>) -> &[Instruction] {
        let range = Range { start: span.start as usize, end: span.end as usize };
        if cfg!(debug_assertions) {
            self.patterns.get(range).expect("invalid range")
        }
        else {
            unsafe { self.patterns.get_unchecked(range) }
        }
    }
    pub fn rule_sequence(&self, _: u32, _: &Range<u32>) -> &[Instruction] {
        todo!()
    }
}

pub struct CharacterRange {
    pub negative: bool,
    pub min: char,
    pub max: char,
}

pub struct StringPattern {
    pub insensitive: bool,
    pub string: CharwiseDoubleArrayAhoCorasick<String>,
}
