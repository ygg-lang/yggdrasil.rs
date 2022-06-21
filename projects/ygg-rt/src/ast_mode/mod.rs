use std::{ops::Range, slice::SliceIndex};

use ucd_trie::TrieSet;

use crate::{results::StopBecause, SResult, SResult::Pending};

mod advance;
mod builtin;
mod choice;
mod concat;

/// The state of parsing
#[derive(Copy, Clone, Debug)]
pub struct YState<'i> {
    /// Rest part of string
    pub partial_string: &'i str,
    /// Start offset of the string
    pub start_offset: usize,
    /// Stop reason
    pub stop_reason: Option<StopBecause>,
}

impl<'i> YState<'i> {
    /// Create a new state
    #[inline(always)]
    pub fn new(input: &'i str) -> Self {
        Self { partial_string: input, start_offset: 0, stop_reason: None }
    }
    /// Reset the cursor offset
    #[inline(always)]
    pub fn with_start_offset(mut self, offset: usize) -> Self {
        self.start_offset = offset;
        self
    }
    /// Finish with given value
    #[inline(always)]
    pub fn finish<T>(self, value: T) -> SResult<'i, T> {
        Pending(self, value)
    }
    /// Check if the string is depleted
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.partial_string.is_empty()
    }
    /// Get inner error
    #[inline(always)]
    pub fn get_error(self) -> StopBecause {
        match self.stop_reason {
            Some(s) => s,
            None => StopBecause::Uninitialized,
        }
    }
    /// Set inner error
    #[inline(always)]
    pub fn set_error(&mut self, error: StopBecause) {
        self.stop_reason = Some(error);
    }
    /// Get a string view
    #[inline(always)]
    pub fn get_string<R>(&self, range: R) -> Option<&R::Output>
    where
        R: SliceIndex<str>,
    {
        self.partial_string.get(range)
    }
    /// Get nth character
    #[inline(always)]
    pub fn get_character(&self, nth: usize) -> Option<char> {
        self.partial_string.chars().nth(nth)
    }
    /// Get range away from start state
    #[inline(always)]
    pub fn away_from(&self, start: YState) -> Range<usize> {
        start.start_offset..self.start_offset
    }
}
