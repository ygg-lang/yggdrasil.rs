pub use self::builtin::{parse_char, parse_string_literal};
use crate::results::{IError, IResult};
use std::slice::SliceIndex;

mod advance;
mod builtin;
mod choice;

#[derive(Debug)]
pub struct Parsed<'i, T> {
    pub term: T,
    pub rest: ParseState<'i>,
}

#[derive(Debug)]
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
