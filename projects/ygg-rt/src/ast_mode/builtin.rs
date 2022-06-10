use super::*;

impl<'i> YState<'i> {
    /// Parses a single character.
    pub fn match_char(self, target: char) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c.eq(&target) => self.advance(target).finish(target),
            _ => Err(StopBecause::MissingCharacter { expected: target, position: self.start_offset })?,
        }
    }
    #[inline]
    pub fn match_char_range(self, start: char, end: char) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c <= end && c >= start => self.advance(c).finish(c),
            _ => Err(StopBecause::MissingCharacterRange { start, end, position: self.start_offset }),
        }
    }
    #[inline]
    pub fn match_char_if(self, predicate: impl Fn(char) -> bool, message: &'static str) -> YResult<'i, char> {
        match self.get_character(0) {
            Some(c) if predicate(c) => self.advance(c).finish(c),
            _ => Err(StopBecause::MissingString { message, position: self.start_offset })?,
        }
    }
    #[inline]
    pub fn parse_char_set(self, set: TrieSet, message: &'static str) -> YResult<'i, char> {
        self.match_char_if(|c| set.contains_char(c), message)
    }
    #[inline]
    pub fn match_string(self, target: &'static str, insensitive: bool) -> YResult<'i, &'static str> {
        match self.get_string(0..target.len()) {
            Some(s) if insensitive && s.eq_ignore_ascii_case(target) => {}
            Some(s) if s.eq(target) => {}
            _ => Err(StopBecause::MissingString { message: target, position: self.start_offset })?,
        }
        self.advance(target).finish(target)
    }
    #[inline]
    pub fn match_string_if(self, predicate: impl Fn(char) -> bool, message: &'static str) -> YResult<'i, String> {
        let mut offset = 0;
        let mut result = String::new();
        for char in self.partial_string.chars() {
            if predicate(char) {
                result.push(char);
                offset += char.len_utf8();
            }
            else {
                break;
            }
        }
        if offset == 0 {
            Err(StopBecause::MissingString { message, position: self.start_offset })?;
        }
        self.advance(offset).finish(result)
    }

    #[inline]
    pub fn match_eof(self) -> YResult<'i, ()> {
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
            let (new, value) = match parse(state) {
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
    /// p* <=> p{0, \inf}
    /// p+ <=> p{1, \inf}
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
    pub fn match_optional<T, F>(self, parse: F) -> YResult<'i, Option<T>>
    where
        F: Fn(YState) -> YResult<T>,
    {
        match parse(self.clone()) {
            Ok((state, value)) => state.finish(Some(value)),
            Err(_) => self.finish(None),
        }
    }
    #[inline]
    pub fn skip<F, T>(self, parse: F) -> YState<'i>
    where
        F: Fn(YState) -> YResult<T>,
    {
        match parse(self.clone()) {
            Ok((new, _)) => new,
            Err(_) => self,
        }
    }

    /// Parse without consuming post-assertions
    /// ```regex
    /// !ahead p
    /// p !after
    /// ```
    pub fn match_negative<F, T>(self, parse: F, message: &'static str) -> YResult<'i, ()>
    where
        F: Fn(YState) -> YResult<T>,
    {
        match parse(self.clone()) {
            Ok(_) => Err(StopBecause::ShouldNotBe { message, position: self.start_offset }),
            Err(_) => self.finish(()),
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
    pub fn match_comment_line(self, head: &'static str) -> YResult<'i, ()> {
        if !self.partial_string.starts_with(head) {
            Err(StopBecause::MissingString { message: head, position: self.start_offset })?;
        }
        let mut offset = head.len();
        let mut rest = self.partial_string[offset..].chars();
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
        self.advance(offset).finish(())
    }
    /// Parse the comment block
    ///
    /// ```ygg
    /// /* */
    /// ```
    pub fn match_comment_block<F, T>(self, head: &'static str, tail: &'static str, nested: bool) -> YResult<'i, ()>
    where
        F: Fn(YState) -> YResult<T>,
    {
        if !self.partial_string.starts_with(head) {
            Err(StopBecause::MissingString { message: head, position: self.start_offset })?;
        }
        let mut offset = head.len();
        let mut _rest = self.partial_string[offset..].chars();
        todo!();
        self.advance(offset).finish(())
    }
}
