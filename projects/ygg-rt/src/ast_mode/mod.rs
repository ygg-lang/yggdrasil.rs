use std::slice::SliceIndex;

use crate::results::StopBecause;

mod advance;
mod builtin;
mod choice;
mod concat;

pub type YResult<'i, T> = Result<Parsed<'i, T>, StopBecause>;

pub enum YYResult<'i, T> {
    Pending(YState<'i>, T),
    Stop(StopBecause),
}

pub type Parsed<'i, T> = (YState<'i>, T);

#[derive(Debug, Clone)]
pub struct YState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    ///
    pub start_offset: usize,
    ///
    pub stop_reason: Option<StopBecause>,
}

impl<'i> YState<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, stop_reason: None }
    }
    pub fn finish<T>(self, value: T) -> YResult<'i, T> {
        Ok((self, value))
    }
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
    pub fn set_error(&mut self, error: StopBecause) {
        self.stop_reason = Some(error);
    }
    pub fn get_error(self) -> StopBecause {
        match self.stop_reason {
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
