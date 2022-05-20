use std::slice::SliceIndex;

use crate::results::{IError, IResult};

mod advance;
mod builtin;
mod choice;
mod concat;

#[derive(Debug, Clone)]
pub struct Parsed<'i, T> {
    pub value: T,
    pub state: ParseState<'i>,
}

#[derive(Debug, Clone)]
pub struct ParseState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    ///
    pub start_offset: usize,
    ///
    pub farthest_error: Option<IError>,
}

impl<'i> ParseState<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, farthest_error: None }
    }
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
    pub fn set_error(&mut self, error: IError) {
        self.farthest_error = Some(error);
    }
    pub fn get_error(self) -> IError {
        match self.farthest_error {
            Some(s) => s,
            None => IError::uninitialized(""),
        }
    }

    pub fn get_string<R>(&self, range: R) -> Option<&R::Output>
    where
        R: SliceIndex<str>,
    {
        self.partial_string.get(range)
    }
    pub fn get_character(&self, nth: usize) -> Option<char> {
        self.partial_string.chars().nth(nth)
    }
}
