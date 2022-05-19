pub struct ParseError {}

pub enum IResult<'i, I> {
    Pending { state: ParseState<'i> },
    Accept { value: T },
    Reject { error: ParseError },
}

pub struct ParseState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    pub start_offset: usize,
    pub farthest_error: Option<ParseError>,
}

pub enum ParseAdvance {
    Offset(usize),
    Character(char),
    String(&'static str),
}

impl<'i> ParseState<'i> {
    pub fn report_error(&self) {}
    pub fn advance<T>(self, term: T) -> ParseState<'i>
    where
        T: Into<ParseAdvance>,
    {
        let offset = match term.into() {
            ParseAdvance::Offset(v) => v,
            ParseAdvance::Character(v) => v.len_utf8(),
            ParseAdvance::String(v) => v.len(),
        };
        ParseState {
            partial_string: &self.partial_string[offset..],
            start_offset: self.start_offset + offset,
            farthest_error: self.farthest_error,
        }
    }
    pub fn advance_by_char(self, c: char) -> ParseState<'i> {
        self.advance(c.len_utf8())
    }
}

pub fn match_char<'i>(state: ParseState<'i>, c: char) -> IResult<char> {
    if c.is_ascii() {
        // fast path
        if state.is_empty() || state.s().as_bytes()[0] != c as u8 {
            IResult::Reject(state.report_error(ParseErrorSpecifics::ExpectedCharacter { c }))
        }
        else {
            // SAFETY:
            // Callers of this function are responsible that these preconditions are satisfied:
            //    Indexes must lie on UTF-8 sequence boundaries.
            //
            // The byte we are skipping is ASCII, so we are OK.
            let state = unsafe { state.advance(1) };
            Ok(IResult::Accept { result: c, state })
        }
    }
    else if !state.s().starts_with(c) {
        // utf-8 path
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacter { c }))
    }
    else {
        // Skip a full character should be OK.
        let state = unsafe { state.advance_by_char(c) };
        Ok(ParseOk { result: c, state })
    }
}
