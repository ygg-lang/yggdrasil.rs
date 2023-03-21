use super::*;

mod display;

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

impl<'i, R: Clone> Clone for TokenStream<'i, R> {
    fn clone(&self) -> TokenStream<'i, R> {
        TokenStream { queue: Rc::clone(&self.queue), input: self.input, start: self.start, end: self.end }
    }
}
