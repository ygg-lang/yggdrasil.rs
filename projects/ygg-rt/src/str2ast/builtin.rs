use ucd_trie::TrieSet;

use super::*;

impl<'i> ParseState<'i> {
    /// Parses a single character.
    pub fn parse_char(self, target: char) -> IResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c.eq(&target) => Ok(Parsed { value: target, state: self.advance(target) }),
            _ => Err(IError::excepted_character(target)),
        }
    }
    pub fn parse_char_range(self, start: char, end: char) -> IResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c >= start && c <= end => Ok(Parsed { value: c, state: self.advance(c) }),
            _ => Err(IError::excepted_character_range(start, end)),
        }
    }
    pub fn parse_char_set(self, set: TrieSet, name: &'static str) -> IResult<'i, char> {
        match self.get_character(0) {
            Some(c) if set.contains_char(c) => Ok(Parsed { value: c, state: self.advance(c) }),
            _ => Err(IError::excepted_string(name)),
        }
    }
    pub fn parse_string_literal(self, target: &'static str, insensitive: bool) -> IResult<'i, &'static str> {
        match self.get_string(0..target.len()) {
            Some(s) if insensitive && s.eq_ignore_ascii_case(target) => {
                Ok(Parsed { value: target, state: self.advance(target) })
            }
            Some(s) if s.eq(target) => Ok(Parsed { value: target, state: self.advance(target) }),
            _ => Err(IError::excepted_string(target)),
        }
    }
    pub fn parse_eof(self) -> IResult<'i, ()> {
        match self.get_character(0) {
            Some(_) => Err(IError::excepted_character('\0')),
            None => Ok(Parsed { value: (), state: self }),
        }
    }
    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+
    /// p{min, max}
    /// ```
    pub fn parse_repeats<T, F>(self, min: usize, max: usize, parse: F) -> IResult<'i, Vec<T>>
    where
        F: Fn(ParseState) -> IResult<T>,
    {
        let mut result = Vec::new();
        let mut count = 0;
        let mut old = self;
        loop {
            let Parsed { value, state: new } = match parse(old.clone()) {
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
        if count < min { Err(old.get_error()) } else { Ok(Parsed { value: result, state: old }) }
    }
    pub fn parse_optional<T, F>(self, parse: F) -> IResult<'i, Option<T>>
    where
        F: Fn(ParseState) -> IResult<T>,
    {
        match parse(self.clone()) {
            Ok(Parsed { value, state }) => Ok(Parsed { value: Some(value), state }),
            Err(_) => Ok(Parsed { value: None, state: self }),
        }
    }
}
