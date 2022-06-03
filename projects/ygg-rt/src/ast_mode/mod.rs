use std::slice::SliceIndex;

use crate::results::StopBecause;

mod advance;
mod builtin;
mod choice;
mod concat;

pub type YResult<'i, T> = Result<Parsed<'i, T>, StopBecause>;

#[derive(Debug, Clone)]
pub struct Parsed<'i, T>(pub YState<'i>, pub T);

#[derive(Debug, Clone)]
pub struct YState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    ///
    pub start_offset: usize,
    ///
    pub farthest_error: Option<StopBecause>,
}

impl<'i, T> Parsed<'i, T> {
    pub fn ok(state: YState<'i>, value: T) -> YResult<T> {
        Ok(Self(state, value))
    }
}

impl<'i> YState<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, farthest_error: None }
    }
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
    pub fn set_error(&mut self, error: StopBecause) {
        self.farthest_error = Some(error);
    }
    pub fn get_error(self) -> StopBecause {
        match self.farthest_error {
            Some(s) => s,
            None => StopBecause::Uninitialized,
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
