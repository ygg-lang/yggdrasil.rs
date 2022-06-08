use ucd_trie::TrieSet;

use super::*;

impl<'i> YState<'i> {
    /// Parses a single character.
    pub fn parse_char(self, target: char) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c.eq(&target) => self.advance(target).finish(target),
            _ => Err(StopBecause::MissingCharacter { expected: target, position: self.start_offset })?,
        }
    }
    pub fn parse_char_range(self, start: char, end: char) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c <= end && c >= start => self.advance(c).finish(c),
            _ => Err(StopBecause::MissingCharacterRange { start, end, position: self.start_offset }),
        }
    }
    pub fn parse_char_set(self, set: TrieSet, name: &'static str) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if set.contains_char(c) => self.advance(c).finish(c),
            _ => Err(StopBecause::MissingString { string: name, position: self.start_offset }),
        }
    }
    pub fn parse_string_literal(self, target: &'static str, insensitive: bool) -> YResult<'i, &'static str> {
        match self.get_string(0..target.len()) {
            Some(s) if insensitive && s.eq_ignore_ascii_case(target) => {}
            Some(s) if s.eq(target) => {}
            _ => Err(StopBecause::MissingString { string: target, position: self.start_offset })?,
        }
        self.advance(target).finish(target)
    }
    pub fn parse_eof(self) -> YResult<'i, ()> {
        match self.get_character(0) {
            Some(_) => Err(StopBecause::MissingEof { position: self.start_offset }),
            None => self.finish(()),
        }
    }
    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+ <=> p p*
    /// ```
    #[inline]
    pub fn match_repeats<T, F>(self, parse: F) -> YResult<'i, Vec<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        let mut result = Vec::new();
        let mut state = self;
        loop {
            let (new, value) = match parse(state.clone()) {
                Ok(o) => o,
                Err(_) => break,
            };
            state = new;
            result.push(value);
        }
        state.finish(result)
    }

    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+
    /// p{min, max}
    /// ```
    #[inline]
    pub fn match_repeat_m_n<T, F>(self, min: usize, max: usize, parse: F) -> YResult<'i, Vec<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        let mut result = Vec::new();
        let mut count = 0;
        let position = self.start_offset;
        let mut state = self;
        loop {
            let (new, value) = match parse(state.clone()) {
                Ok(o) => o,
                Err(_) => break,
            };
            state = new;
            result.push(value);
            count += 1;
            if count >= max {
                break;
            }
        }
        if count < min {
            Err(StopBecause::MissingRepeats { min, current: count, position })?
        }
        state.finish(result)
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
            Ok((state, value)) => state.finish(Some(value)),
            Err(_) => self.finish(None),
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
            Err(_) => self.finish(()),
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
        let (state, _) = start(self.clone())?;
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
        state.advance(offset).finish(())
    }
}
