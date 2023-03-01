// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use alloc::{rc::Rc, vec::Vec};
use core::fmt;

use super::{
    pair::{self, Pair},
    queueable_token::TokenQueue,
    tokens::{self, Tokens},
};
use crate::YggdrasilRule;

/// An iterator over [`Pair`]s. It is created by [`Pairs::flatten`].
///
/// [`Pair`]: struct.Pair.html
/// [`Pairs::flatten`]: struct.Pairs.html#method.flatten
pub struct TokenStream<'i, R> {
    /// # Safety
    ///
    /// All `QueueableToken`s' `input_pos` must be valid character boundary indices into `input`.
    queue: Rc<Vec<TokenQueue<R>>>,
    input: &'i str,
    start: usize,
    end: usize,
}

/// # Safety
///
/// All `QueueableToken`s' `input_pos` must be valid character boundary indices into `input`.
pub unsafe fn new<R: YggdrasilRule>(queue: Rc<Vec<TokenQueue<R>>>, input: &str, start: usize, end: usize) -> TokenStream<R> {
    TokenStream { queue, input, start, end }
}

impl<'i, R: YggdrasilRule> TokenStream<'i, R> {
    /// Returns the `Tokens` for these pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    /// }
    ///
    /// let input = "";
    /// let pairs = pest::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// })
    /// .unwrap();
    /// let tokens: Vec<_> = pairs.flatten().tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 2);
    /// ```
    #[inline]
    pub fn tokens(self) -> Tokens<'i, R> {
        tokens::new(self.queue, self.input, self.start, self.end)
    }

    fn next_start(&mut self) {
        self.start += 1;

        while self.start < self.end && !self.is_start(self.start) {
            self.start += 1;
        }
    }

    fn next_start_from_end(&mut self) {
        self.end -= 1;

        while self.end >= self.start && !self.is_start(self.end) {
            self.end -= 1;
        }
    }

    fn is_start(&self, index: usize) -> bool {
        match self.queue[index] {
            TokenQueue::Start { .. } => true,
            TokenQueue::End { .. } => false,
        }
    }
}

impl<'i, R: YggdrasilRule> ExactSizeIterator for TokenStream<'i, R> {
    fn len(&self) -> usize {
        // Tokens len is exactly twice as flatten pairs len
        (self.end - self.start) >> 1
    }
}

impl<'i, R: YggdrasilRule> Iterator for TokenStream<'i, R> {
    type Item = Pair<'i, R>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let pair = unsafe { pair::new(Rc::clone(&self.queue), self.input, self.start) };
        self.next_start();

        Some(pair)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = <Self as ExactSizeIterator>::len(self);
        (len, Some(len))
    }
}

impl<'i, R: YggdrasilRule> DoubleEndedIterator for TokenStream<'i, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        self.next_start_from_end();

        let pair = unsafe { pair::new(Rc::clone(&self.queue), self.input, self.end) };

        Some(pair)
    }
}

impl<'i, R: YggdrasilRule> fmt::Debug for TokenStream<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatPairs").field("pairs", &self.clone().collect::<Vec<_>>()).finish()
    }
}

impl<'i, R: Clone> Clone for TokenStream<'i, R> {
    fn clone(&self) -> TokenStream<'i, R> {
        TokenStream { queue: Rc::clone(&self.queue), input: self.input, start: self.start, end: self.end }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{macros::tests::*, YggdrasilLanguage};
    use alloc::{vec, vec::Vec};

    #[test]
    fn iter_for_flat_pairs() {
        let pairs = AbcParser::parse_cst(TestRule::a, "abcde").unwrap();

        assert_eq!(
            pairs.flatten().map(|p| p.as_rule()).collect::<Vec<TestRule>>(),
            vec![TestRule::a, TestRule::b, TestRule::c]
        );
    }

    #[test]
    fn double_ended_iter_for_flat_pairs() {
        let pairs = AbcParser::parse_cst(TestRule::a, "abcde").unwrap();
        assert_eq!(
            pairs.flatten().rev().map(|p| p.as_rule()).collect::<Vec<TestRule>>(),
            vec![TestRule::c, TestRule::b, TestRule::a]
        );
    }

    #[test]
    fn test_line_col() {
        let mut pairs = AbcParser::parse_cst(TestRule::a, "abcNe\nabcde").unwrap().flatten();

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "abc");
        assert_eq!(pair.line_col(), (1, 1));
        assert_eq!(pair.line_col(), pair.as_span().start_pos().line_column());

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "b");
        assert_eq!(pair.line_col(), (1, 2));
        assert_eq!(pair.line_col(), pair.as_span().start_pos().line_column());

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "e");
        assert_eq!(pair.line_col(), (1, 5));
        assert_eq!(pair.line_col(), pair.as_span().start_pos().line_column());
    }

    #[test]
    fn exact_size_iter_for_pairs() {
        let pairs = AbcParser::parse_cst(TestRule::a, "abc\nefgh").unwrap().flatten();
        assert_eq!(pairs.len(), pairs.count());

        let pairs = AbcParser::parse_cst(TestRule::a, "我很漂亮efgh").unwrap().flatten();
        assert_eq!(pairs.len(), pairs.count());

        let pairs = AbcParser::parse_cst(TestRule::a, "abc\nefgh").unwrap().flatten();
        let pairs = pairs.rev();
        assert_eq!(pairs.len(), pairs.count());

        let mut pairs = AbcParser::parse_cst(TestRule::a, "abc\nefgh").unwrap().flatten();
        let pairs_len = pairs.len();
        let _ = pairs.next().unwrap();
        assert_eq!(pairs.count() + 1, pairs_len);
    }
}
