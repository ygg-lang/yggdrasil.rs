use crate::{SResult, SResult::Stop};

use super::*;

impl<'i> YState<'i> {
    /// Match a single character.
    ///
    /// ```ygg
    /// 'c'
    /// ```
    #[inline]
    pub fn match_char(self, target: char) -> SResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c.eq(&target) => self.advance(target).finish(target),
            _ => Stop(StopBecause::MissingCharacter { expected: target, position: self.start_offset }),
        }
    }
    /// Match a character in given range.
    ///
    /// ```ygg
    /// [a-z]
    /// ```
    #[inline]
    pub fn match_char_range(self, start: char, end: char) -> SResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c <= end && c >= start => self.advance(c).finish(c),
            _ => Stop(StopBecause::MissingCharacterRange { start, end, position: self.start_offset }),
        }
    }
    /// Parsing a character with given rule.
    #[inline]
    pub fn match_char_if(self, predicate: impl Fn(char) -> bool, message: &'static str) -> SResult<'i, char> {
        match self.get_character(0) {
            Some(c) if predicate(c) => self.advance(c).finish(c),
            _ => Stop(StopBecause::MustBe { message, position: self.start_offset }),
        }
    }
    /// Match any character, except `EOF`.
    #[inline]
    pub fn match_char_any(self) -> SResult<'i, char> {
        self.match_char_if(|_| true, "ANY")
    }
    /// Match a character with given set.
    #[inline]
    pub fn parse_char_set(self, set: TrieSet, message: &'static str) -> SResult<'i, char> {
        self.match_char_if(|c| set.contains_char(c), message)
    }
    /// Match a static string.
    #[inline]
    pub fn match_str_static(self, target: &'static str, insensitive: bool) -> SResult<'i, &'i str> {
        let s = match self.get_string(0..target.len()) {
            Some(s) if insensitive && s.eq_ignore_ascii_case(target) => s.len(),
            Some(s) if s.eq(target) => s.len(),
            _ => Stop(StopBecause::MissingString { message: target, position: self.start_offset })?.1,
        };
        self.advance_view(s)
    }
    /// Match a string with given rule.
    #[inline]
    pub fn match_str_if(self, predicate: impl Fn(char) -> bool, message: &'static str) -> SResult<'i, &'i str> {
        let mut offset = 0;
        for char in self.partial_string.chars() {
            match predicate(char) {
                true => offset += char.len_utf8(),
                false => break,
            }
        }
        if offset == 0 {
            Err(StopBecause::MissingString { message, position: self.start_offset })?;
        }
        self.advance(offset).finish(&self.partial_string[..offset])
    }
    /// Assert end of file
    /// ```ygg
    /// p $
    /// ```
    #[inline]
    pub fn match_eof(self) -> SResult<'i, ()> {
        match self.get_character(0) {
            Some(_) => Stop(StopBecause::ExpectEof { position: self.start_offset })?.1,
            None => self.finish(()),
        }
    }
    /// Simple suffix call form
    #[inline]
    pub fn match_parse<T, F>(self, parse: F) -> SResult<'i, T>
    where
        F: Fn(YState) -> SResult<T>,
    {
        parse(self)
    }
    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+ <=> p p*
    /// ```
    #[inline]
    pub fn match_repeats<T, F>(self, parse: F) -> SResult<'i, Vec<T>>
    where
        F: Fn(YState) -> SResult<T>,
    {
        let mut result = Vec::new();
        let mut state = self;
        loop {
            let (new, value) = match parse(state) {
                Pending(o, v) => (o, v),
                Stop(_) => break,
            };
            state = new;
            result.push(value);
        }
        state.finish(result)
    }

    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p* <=> p{0, \inf}
    /// p+ <=> p{1, \inf}
    /// p{min, max}
    /// ```
    #[inline]
    pub fn match_repeat_m_n<T, F>(self, min: usize, max: usize, parse: F) -> SResult<'i, Vec<T>>
    where
        F: Fn(YState) -> SResult<T>,
    {
        let mut result = Vec::new();
        let mut count = 0;
        let position = self.start_offset;
        let mut state = self;
        loop {
            let (new, value) = match parse(state.clone()) {
                Pending(o, v) => (o, v),
                Stop(_) => break,
            };
            state = new;
            result.push(value);
            count += 1;
            if count >= max {
                break;
            }
        }
        if count < min {
            Err(StopBecause::ExpectRepeats { min, current: count, position })?
        }
        state.finish(result)
    }
    /// Parse an optional element
    /// ```regex
    /// p?
    /// ```
    #[inline]
    pub fn match_optional<T, F>(self, parse: F) -> SResult<'i, Option<T>>
    where
        F: Fn(YState) -> SResult<T>,
    {
        match parse(self.clone()) {
            Pending(state, value) => state.finish(Some(value)),
            Stop(_) => self.finish(None),
        }
    }
    /// Match but does not return the result
    #[inline]
    pub fn skip<F, T>(self, parse: F) -> YState<'i>
    where
        F: Fn(YState) -> SResult<T>,
    {
        match parse(self.clone()) {
            Pending(new, _) => new,
            Stop(_) => self,
        }
    }
    /// Zero-width positive match, does not consume input
    ///
    /// Used to be a external rule, which used as assert
    ///
    /// ```regex
    /// &ahead p
    /// p &after
    /// ```
    #[inline]
    pub fn match_positive<F, T>(self, parse: F, message: &'static str) -> SResult<'i, ()>
    where
        F: Fn(YState) -> SResult<T>,
    {
        match parse(self.clone()) {
            Pending(..) => self.finish(()),
            Stop(_) => Stop(StopBecause::MustBe { message, position: self.start_offset }),
        }
    }
    /// Zero-width negative match, does not consume input
    /// ```regex
    /// !ahead p
    /// p !after
    /// ```
    #[inline]
    pub fn match_negative<F, T>(self, parse: F, message: &'static str) -> SResult<'i, ()>
    where
        F: Fn(YState) -> SResult<T>,
    {
        match parse(self.clone()) {
            Pending(..) => Stop(StopBecause::ShouldNotBe { message, position: self.start_offset }),
            Stop(_) => self.finish(()),
        }
    }
}

impl<'i> YState<'i> {
    /// Parse a comment line
    /// ```regex
    /// # comment
    /// // comment
    /// ```
    #[inline]
    pub fn match_comment_line(self, head: &'static str) -> SResult<'i, &'i str> {
        if !self.partial_string.starts_with(head) {
            Stop::<&'i str>(StopBecause::MissingString { message: head, position: self.start_offset })?.1;
        }
        let offset = match self.partial_string.find(|c| c == '\r' || c == '\n') {
            Some(s) => s,
            None => self.partial_string.len(),
        };
        self.advance(offset).finish(&self.partial_string[..offset])
    }
    /// Parse the comment block
    ///
    /// ```ygg
    /// /* */
    /// ```
    #[inline]
    pub fn match_comment_block<F, T>(self, head: &'static str, tail: &'static str) -> SResult<'i, ()>
    where
        F: Fn(YState) -> SResult<T>,
    {
        if !self.partial_string.starts_with(head) {
            Stop::<()>(StopBecause::MissingString { message: head, position: self.start_offset })?;
        }
        let mut offset = head.len();
        let mut rest = &self.partial_string[offset..];
        match rest.find(tail) {
            Some(s) => offset += s + tail.len(),
            None => Stop(StopBecause::MissingString { message: tail, position: self.start_offset + tail.len() })?.1,
        }
        self.advance(offset).finish(())
    }

    /// Parse the comment block
    ///
    /// ```ygg
    /// r#" "#
    /// r##" "##
    /// r###" "###
    /// ```
    pub fn match_surround<F, T>(self, delimiter: char, min: usize) -> SResult<'i, ()>
    where
        F: Fn(YState) -> SResult<T>,
    {
        let mut count = 0;
        for c in self.partial_string.chars() {
            match c == delimiter {
                true => count += 1,
                false => break,
            }
        }
        if count == 0 {
            Stop::<()>(StopBecause::MissingString { message: "r#", position: self.start_offset })?;
        }
        if count < min {
            Stop::<()>(StopBecause::MissingString { message: "r##", position: self.start_offset })?;
        }
        let head = count * delimiter.len_utf8();
        let rest = &self.partial_string[head..];
        let end = delimiter.to_string().repeat(count);
        match rest.find(&end) {
            Some(s) => self.advance(s + count * delimiter.len_utf8()).finish(()),
            None => Stop(StopBecause::MissingString { message: "match_raw_paired", position: self.start_offset + count }),
        }
    }
}
