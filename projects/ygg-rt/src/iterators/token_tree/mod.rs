use super::*;

mod display;

/// An iterator over [`Pair`]s. It is created by [`yggdrasil_rt::state`] and [`Pair::into_inner`].
///
/// [`Pair`]: struct.Pair.html
/// [`yggdrasil_rt::state`]: ../fn.state.html
/// [`Pair::into_inner`]: struct.Pair.html#method.into_inner
#[derive(Clone)]
pub struct TokenTree<'i, R> {
    queue: Rc<Vec<TokenQueue<R>>>,
    input: &'i str,
    start: usize,
    end: usize,
    pairs_count: usize,
}

impl<'i, R> TokenTree<'i, R>
where
    R: YggdrasilRule,
{
    /// Create a new token tree from token stream
    pub fn new(queue: Rc<Vec<TokenQueue<R>>>, input: &'i str, start: usize, end: usize) -> Self {
        let mut pairs_count = 0;
        let mut cursor = start;
        while cursor < end {
            cursor = match queue[cursor] {
                TokenQueue::Start { end_token_index, .. } => end_token_index,
                _ => unreachable!(),
            } + 1;
            pairs_count += 1;
        }
        Self { queue, input, start, end, pairs_count }
    }
}

impl<'i, R> TokenTree<'i, R>
where
    R: YggdrasilRule + Clone,
{
    /// Captures a slice from the `&str` defined by the starting position of the first token `Pair`
    /// and the ending position of the last token `Pair` of the `Pairs`. This also captures
    /// the input between those two token `Pair`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt::{state};
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b,
    /// }
    ///
    /// let input = "a b";
    /// let pairs = state(input, |state| {
    ///     // generating Token pairs with Rule::a and Rule::b ...
    /// #     state.rule(Rule::a, |s| s.match_string("a", false)).and_then(|s| s.skip(1))
    /// #         .and_then(|s| s.rule(Rule::b, |s| s.match_string("b")))
    /// })
    /// .unwrap();
    ///
    /// assert_eq!(pairs.as_str(), "a b");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'i str {
        if self.start < self.end {
            let start = self.pos(self.start);
            let end = self.pos(self.end - 1);
            // Generated positions always come from Positions and are UTF-8 borders.
            &self.input[start..end]
        }
        else {
            ""
        }
    }

    /// Returns the input string of `Pairs`.
    ///
    /// This function returns the input string of `Pairs` as a `&str`. This is the source string
    /// from which `Pairs` was created. The returned `&str` can be used to examine the contents of
    /// `Pairs` or to perform further processing on the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b,
    /// }
    ///
    /// // Example: Get input string from Pairs
    ///
    /// let input = "a b";
    /// let pairs = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pairs with Rule::a and Rule::b ...
    /// #     state.rule(Rule::a, |s| s.match_string("a")).and_then(|s| s.skip(1))
    /// #         .and_then(|s| s.rule(Rule::b, |s| s.match_string("b")))
    /// })
    /// .unwrap();
    ///
    /// assert_eq!(pairs.as_str(), "a b");
    /// assert_eq!(input, pairs.get_input());
    /// ```
    pub fn get_input(&self) -> &'i str {
        self.input
    }

    /// Captures inner token `Pair`s and concatenates resulting `&str`s. This does not capture
    /// the input between token `Pair`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b,
    /// }
    ///
    /// let input = "a b";
    /// let pairs = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pairs with Rule::a and Rule::b ...
    /// #     state.rule(Rule::a, |s| s.match_string("a")).and_then(|s| s.skip(1))
    /// #         .and_then(|s| s.rule(Rule::b, |s| s.match_string("b")))
    /// })
    /// .unwrap();
    ///
    /// assert_eq!(pairs.concat(), "ab");
    /// ```
    #[inline]
    pub fn concat(&self) -> String {
        self.clone().fold(String::new(), |string, pair| string + pair.as_str())
    }

    /// Flattens the `Pairs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b,
    /// }
    ///
    /// let input = "";
    /// let pairs = yggdrasil_rt::state(input, |state| {
    ///     // generating nested Token pair with Rule::b inside Rule::a
    /// #     state.rule(Rule::a, |state| {
    /// #         state.rule(Rule::b, |s| Ok(s))
    /// #     })
    /// })
    /// .unwrap();
    /// let tokens: Vec<_> = pairs.flatten().tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 4);
    /// ```
    #[inline]
    pub fn flatten(self) -> TokenStream<'i, R> {
        unsafe { token_stream::new(self.queue, self.input, self.start, self.end) }
    }

    /// Returns the iterator over pairs that have their node or branch tagged
    /// with the provided label. The iterator is built from a flattened [`TokenTree`] iterator.
    ///
    /// # Examples
    ///
    /// Try to recognize the node between left and right hand side
    /// ```
    /// use yggdrasil_rt::{state, Either, State};
    /// #[allow(non_camel_case_types)]
    /// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     number, // 0..9
    ///     add,    // num + num
    ///     mul,    // num * num
    /// }
    /// fn mark_branch(state: Box<State<'_, Rule>>) -> Either<Box<State<'_, Rule>>> {
    ///     expr(state, Rule::mul, "*")
    ///         .and_then(|state| state.tag_node("mul"))
    ///         .or_else(|state| expr(state, Rule::add, "+"))
    ///         .and_then(|state| state.tag_node("add"))
    /// }
    /// fn expr<'a>(
    ///     state: Box<State<'a, Rule>>,
    ///     r: Rule,
    ///     o: &'static str,
    /// ) -> Either<Box<State<'a, Rule>>> {
    ///     state.rule(r, |state| {
    ///         state.sequence(|state| {
    ///             number(state)
    ///                 .and_then(|state| state.tag_node("lhs"))
    ///                 .and_then(|state| state.match_string(o))
    ///                 .and_then(number)
    ///                 .and_then(|state| state.tag_node("rhs"))
    ///         })
    ///     })
    /// }
    /// fn number(state: Box<State<'_, Rule>>) -> Either<Box<State<'_, Rule>>> {
    ///     state.rule(Rule::number, |state| state.match_range('0'..'9'))
    /// }
    ///
    /// let input = "1+2";
    /// let pairs = state(input, mark_branch).unwrap();
    /// let mut left_numbers = pairs.find_tagged("lhs");
    /// assert_eq!(left_numbers.next().unwrap().as_str(), "1");
    /// assert_eq!(left_numbers.next(), None);
    /// ```
    #[inline]
    pub fn find_tagged(self, tag: &str) -> Filter<TokenStream<'i, R>, impl FnMut(&TokenPair<'i, R>) -> bool + '_> {
        self.flatten().filter(move |pair: &TokenPair<'i, R>| pair.get_tag().eq(tag))
    }

    /// Returns the `Tokens` for the `Pairs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    /// }
    ///
    /// let input = "";
    /// let pairs = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// })
    /// .unwrap();
    /// let tokens: Vec<_> = pairs.tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 2);
    /// ```
    #[inline]
    pub fn tokens(self) -> Tokens<'i, R> {
        tokens::new(self.queue, self.input, self.start, self.end)
    }

    /// Peek at the first inner `Pair` without changing the position of this iterator.
    #[inline]
    pub fn peek(&self) -> Option<TokenPair<'i, R>> {
        if self.start < self.end {
            Some(unsafe { token_pair::new(Rc::clone(&self.queue), self.input, self.start) })
        }
        else {
            None
        }
    }

    fn pair(&self) -> usize {
        match self.queue[self.start] {
            TokenQueue::Start { end_token_index, .. } => end_token_index,
            _ => unreachable!(),
        }
    }

    fn pair_from_end(&self) -> usize {
        match self.queue[self.end - 1] {
            TokenQueue::End { start_token_index, .. } => start_token_index,
            _ => unreachable!(),
        }
    }

    fn pos(&self, index: usize) -> usize {
        match self.queue[index] {
            TokenQueue::Start { input_offset: input_pos, .. } | TokenQueue::End { input_offset: input_pos, .. } => input_pos,
        }
    }
}

impl<'i, R: YggdrasilRule> ExactSizeIterator for TokenTree<'i, R> {
    #[inline]
    fn len(&self) -> usize {
        self.pairs_count
    }
}

impl<'i, R: YggdrasilRule> Iterator for TokenTree<'i, R> {
    type Item = TokenPair<'i, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.peek()?;

        self.start = self.pair() + 1;
        self.pairs_count -= 1;
        Some(pair)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = <Self as ExactSizeIterator>::len(self);
        (len, Some(len))
    }
}

impl<'i, R: YggdrasilRule> DoubleEndedIterator for TokenTree<'i, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        self.end = self.pair_from_end();
        self.pairs_count -= 1;

        let pair = unsafe { token_pair::new(Rc::clone(&self.queue), self.input, self.end) };

        Some(pair)
    }
}

impl<'i, R: PartialEq> PartialEq for TokenTree<'i, R> {
    fn eq(&self, other: &TokenTree<'i, R>) -> bool {
        Rc::ptr_eq(&self.queue, &other.queue)
            && ptr::eq(self.input, other.input)
            && self.start == other.start
            && self.end == other.end
    }
}

impl<'i, R: Eq> Eq for TokenTree<'i, R> {}

impl<'i, R: Hash> Hash for TokenTree<'i, R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.queue as *const Vec<TokenQueue<R>>).hash(state);
        (self.input as *const str).hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}
