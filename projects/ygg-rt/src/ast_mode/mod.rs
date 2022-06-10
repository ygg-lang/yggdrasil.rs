use std::{ops::Range, slice::SliceIndex};

use crate::results::StopBecause;
use ucd_trie::TrieSet;
mod advance;
mod builtin;
mod choice;
mod concat;

pub type YResult<'i, T> = Result<Parsed<'i, T>, StopBecause>;

/// will change [`YResult`] to [`ZResult`] if try 2.0 is stable
pub enum ZResult<'i, T> {
    Pending(YState<'i>, T),
    Stop(StopBecause),
}

pub type Parsed<'i, T> = (YState<'i>, T);

#[derive(Copy, Clone, Debug)]
pub struct YState<'i> {
    /// reset part of string
    pub partial_string: &'i str,
    ///
    pub start_offset: usize,
    ///
    pub stop_reason: Option<StopBecause>,
}

impl<'i> YState<'i> {
    #[inline(always)]
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, stop_reason: None }
    }
    #[inline(always)]
    pub fn finish<T>(self, value: T) -> YResult<'i, T> {
        Ok((self, value))
    }
    /// Check if the string is depleted
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
    #[inline(always)]
    pub fn set_error(&mut self, error: StopBecause) {
        self.stop_reason = Some(error);
    }
    #[inline(always)]
    pub fn get_error(self) -> StopBecause {
        match self.stop_reason {
            Some(s) => s,
            None => StopBecause::Uninitialized,
        }
    }
    #[inline(always)]
    pub fn get_string<R>(&self, range: R) -> Option<&R::Output>
    where
        R: SliceIndex<str>,
    {
        self.partial_string.get(range)
    }
    #[inline(always)]
    pub fn get_character(&self, nth: usize) -> Option<char> {
        self.partial_string.chars().nth(nth)
    }
    #[inline(always)]
    pub fn away_from(&self, start: YState) -> Range<usize> {
        start.start_offset..self.start_offset
    }
}

impl<'i, T> ZResult<'i, T> {
    #[inline(always)]
    fn map_inner<F, U>(self, f: F) -> ZResult<'i, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            ZResult::Pending(state, value) => ZResult::Pending(state, f(value)),
            ZResult::Stop(reason) => ZResult::Stop(reason),
        }
    }
}
