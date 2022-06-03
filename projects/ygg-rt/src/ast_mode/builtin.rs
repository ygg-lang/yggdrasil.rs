use ucd_trie::TrieSet;

use crate::{CapturedCharacter, CapturedString};

use super::*;

impl<'i> YState<'i> {
    /// Parses a single character.
    pub fn parse_char(self, target: char) -> YResult<'i, CapturedCharacter> {
        match self.get_character(0) {
            Some(c) if c.eq(&target) => {}
            _ => Err(StopBecause::MissingCharacter { expected: target, position: self.start_offset })?,
        }
        let value = CapturedCharacter::new(target, self.start_offset);
        Parsed::ok(self.advance(target), value)
    }
    pub fn parse_char_range(self, start: char, end: char) -> YResult<'i, CapturedCharacter> {
        let offset = self.start_offset;
        match self.get_character(0) {
            Some(c) if c <= end && c >= start => {
                let value = CapturedCharacter::new(c, offset);
                Parsed::ok(self.advance(c), value)
            }
            _ => Err(StopBecause::MissingCharacterRange { start, end, position: self.start_offset }),
        }
    }
    pub fn parse_char_set(self, set: TrieSet, name: &'static str) -> YResult<'i, CapturedCharacter> {
        let offset = self.start_offset;
        match self.get_character(0) {
            Some(c) if set.contains_char(c) => {
                let value = CapturedCharacter::new(c, offset);
                Parsed::ok(self.advance(c), value)
            }
            _ => Err(StopBecause::MissingString { string: name, position: self.start_offset }),
        }
    }
    pub fn parse_string_literal(self, target: &'static str, insensitive: bool) -> YResult<'i, CapturedString> {
        match self.get_string(0..target.len()) {
            Some(s) if insensitive && s.eq_ignore_ascii_case(target) => {}
            Some(s) if s.eq(target) => {}
            _ => Err(StopBecause::MissingString { string: target, position: self.start_offset })?,
        }
        let value = CapturedString::new(target, self.start_offset);
        Parsed::ok(self.advance(target), value)
    }
    pub fn parse_eof(self) -> YResult<'i, ()> {
        match self.get_character(0) {
            Some(_) => Err(StopBecause::MissingEof { position: self.start_offset }),
            None => Parsed::ok(self, ()),
        }
    }
    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+
    /// p{min, max}
    /// ```
    #[inline]
    pub fn parse_repeats<T, F>(self, min: usize, max: usize, parse: F) -> YResult<'i, Vec<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        let mut result = Vec::new();
        let mut count = 0;
        let position = self.start_offset;
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
            Err(StopBecause::MissingRepeats { min, current: count, position })?
        }
        Parsed::ok(old, result)
    }
    /// Parse an optional element
    /// ```regex
    /// p?
    /// ```
    pub fn parse_optional<T, F>(self, parse: F) -> YResult<'i, Option<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        match parse(self.clone()) {
            Ok(Parsed(state, value)) => Parsed::ok(state, Some(value)),
            Err(_) => Parsed::ok(self, None),
        }
    }
    /// Parse without consuming post-assertions
    /// ```regex
    /// !ahead p
    /// p !after
    /// ```
    pub fn parse_negative<F, T>(self, parse: F, name: &'static str) -> YResult<'i, ()>
    where
        F: Fn(YState) -> YResult<T>,
    {
        match parse(self.clone()) {
            Ok(_) => Err(StopBecause::ShouldNotBe { string: name, position: self.start_offset }),
            Err(_) => Parsed::ok(self, ()),
        }
    }
    /// Parse a comment line
    /// ```regex
    /// # comment
    /// // comment
    /// ```
    pub fn parse_comment_line<F, T>(self, start: F) -> YResult<'i, ()>
    where
        F: Fn(YState) -> YResult<T>,
    {
        let Parsed(state, _) = start(self.clone())?;
        let mut offset = 0;
        let mut rest = state.partial_string.chars();
        while let Some(c) = rest.next() {
            match c {
                '\r' => {
                    match rest.next() {
                        Some('\n') => offset += 2,
                        _ => offset += 1,
                    }
                    break;
                }
                '\n' => {
                    offset += 1;
                    break;
                }
                _ => {}
            }
        }
        Parsed::ok(state.advance(offset), ())
    }
}
