use std::slice::SliceIndex;

use crate::results::YError;

mod advance;
mod builtin;
mod choice;
mod concat;

pub type YResult<'i, T> = Result<Parsed<'i, T>, YError>;

#[derive(Debug, Clone)]
pub struct Parsed<'i, T> {
    pub value: T,
    pub state: YState<'i>,
}

#[derive(Debug, Clone)]
pub struct YState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    ///
    pub start_offset: usize,
    ///
    pub farthest_error: Option<YError>,
}

impl<'i, T> Parsed<'i, T> {
    pub fn ok(value: T, state: YState<'i>) -> YResult<T> {
        Ok(Self { value, state })
    }
}

impl<'i> YState<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, farthest_error: None }
    }
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
    pub fn set_error(&mut self, error: YError) {
        self.farthest_error = Some(error);
    }
    pub fn get_error(self) -> YError {
        match self.farthest_error {
            Some(s) => s,
            None => YError::uninitialized(""),
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
