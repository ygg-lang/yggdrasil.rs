use std::ops::Range;

pub mod advance;
pub mod builtin;

#[derive(Debug)]
pub struct YError {}

impl YError {
    pub fn excepted_char(c: char) -> Self {
        Self {}
    }
    pub fn excepted_string(s: &'static str) -> Self {
        Self {}
    }
}

pub type IResult<'i, T> = Result<Parsed<'i, T>, YError>;

#[derive(Debug)]
pub struct Parsed<'i, T> {
    pub term: T,
    pub rest: ParseState<'i>,
}

#[derive(Debug)]
pub struct ParseState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    pub start_offset: usize,
    pub farthest_error: Option<YError>,
}

impl<'i> ParseState<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, farthest_error: None }
    }
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
}
