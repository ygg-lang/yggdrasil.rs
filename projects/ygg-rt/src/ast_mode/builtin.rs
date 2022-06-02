use ucd_trie::TrieSet;

use crate::CapturedCharacter;

use super::*;

impl<'i> YState<'i> {
    /// Parses a single character.
    pub fn parse_char(self, target: char) -> YResult<'i, CapturedCharacter> {
        let start = self.start_offset;
        match self.get_character(0) {
            Some(c) if c.eq(&target) => {
                let value = CapturedCharacter::new(c, start);
                Parsed::ok(self.advance(target), value)
            }
            _ => Err(YError::ExceptedCharacter(target)),
        }
    }
    pub fn parse_char_range(self, start: char, end: char) -> YResult<'i, CapturedCharacter> {
        let offset = self.start_offset;
        match self.get_character(0) {
            Some(c) if c >= start && c <= end => {
                let value = CapturedCharacter::new(c, offset);
                Parsed::ok(self.advance(target), value)
            }
            _ => Err(YError::ExceptedCharacterRange(start, end)),
        }
    }
    pub fn parse_char_set(self, set: TrieSet, name: &'static str) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if set.contains_char(c) => Parsed::ok(self.advance(c), c),
            _ => Err(YError::ExceptedString(name)),
        }
    }
    pub fn parse_string_literal(self, target: &'static str, insensitive: bool) -> YResult<'i, &'static str> {
        match self.get_string(0..target.len()) {
            Some(s) if insensitive && s.eq_ignore_ascii_case(target) => Parsed::ok(self.advance(target), target),
            Some(s) if s.eq(target) => Parsed::ok(self.advance(target), target),
            _ => Err(YError::ExceptedString(target)),
        }
    }
    pub fn parse_eof(self) -> YResult<'i, ()> {
        match self.get_character(0) {
            Some(_) => Err(YError::ExceptedEof),
            None => Parsed::ok(self, ()),
        }
    }
    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+
    /// p{min, max}
    /// ```
    pub fn parse_repeats<T, F>(self, min: usize, max: usize, parse: F) -> YResult<'i, Vec<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        let mut result = Vec::new();
        let mut count = 0;
        let mut old = self;
        loop {
            let Parsed(new, value) = match parse(old.clone()) {
                Ok(o) => o,
                Err(_) => break,
            };
            result.push(value);
            old = new;
            count += 1;
            if count >= max {
                break;
            }
        }
        if count < min {
            Err(YError::ExceptRepeats { min, current: count })?
        }
        Parsed::ok(old, result)
    }
    pub fn parse_optional<T, F>(self, parse: F) -> YResult<'i, Option<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        match parse(self.clone()) {
            Ok(Parsed(state, value)) => Parsed::ok(state, Some(value)),
            Err(_) => Parsed::ok(self, None),
        }
    }
}
