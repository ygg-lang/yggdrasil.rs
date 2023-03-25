use core::{
    fmt,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Bound, RangeBounds},
    ptr, str,
};

use crate::position;

/// A span over a `&str`. It is created from either [two `Position`s] or from a [`Pair`].
///
/// [two `Position`s]: struct.Position.html#method.span
/// [`Pair`]: ../iterators/struct.Pair.html#method.span
#[derive(Clone, Copy)]
pub struct TextSpan<'i> {
    pub(crate) input: &'i str,
    /// # Safety
    ///
    /// Must be a valid character boundary index into `input`.
    pub(crate) start: usize,
    /// # Safety
    ///
    /// Must be a valid character boundary index into `input`.
    pub(crate) end: usize,
}

impl<'i> Display for TextSpan<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let view = &self.input[self.start..self.end];
        write!(f, "({}, {}): \x1b[32m{:?}\x1b[0m", self.start, self.end, view)
    }
}

impl<'i> TextSpan<'i> {
    /// Create a new `Span` without checking invariants. (Checked with `debug_assertions`.)
    ///
    /// # Safety
    ///
    /// `input[start..end]` must be a valid subslice; that is, said indexing should not panic.
    pub(crate) unsafe fn new_unchecked(input: &str, start: usize, end: usize) -> TextSpan<'_> {
        debug_assert!(input.get(start..end).is_some());
        TextSpan { input, start, end }
    }

    /// Attempts to create a new span. Will return `None` if `input[start..end]` is an invalid index
    /// into `input`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::TextSpan;
    /// let input = "Hello!";
    /// assert_eq!(None, TextSpan::new(input, 100, 0));
    /// assert!(TextSpan::new(input, 0, input.len()).is_some());
    /// ```
    pub fn new(input: &str, start: usize, end: usize) -> Option<TextSpan<'_>> {
        if input.get(start..end).is_some() { Some(TextSpan { input, start, end }) } else { None }
    }

    /// Attempts to create a new span based on a sub-range.
    ///
    /// ```
    /// use yggdrasil_rt::TextSpan;
    /// let input = "Hello World!";
    /// let world = TextSpan::new(input, 6, input.len()).unwrap();
    /// let orl = world.get(1..=3);
    /// assert!(orl.is_some());
    /// assert_eq!(orl.unwrap().as_str(), "orl");
    /// ```
    ///
    /// # Examples
    pub fn get(&self, range: impl RangeBounds<usize>) -> Option<TextSpan<'i>> {
        let start = match range.start_bound() {
            Bound::Included(offset) => *offset,
            Bound::Excluded(offset) => *offset + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(offset) => *offset + 1,
            Bound::Excluded(offset) => *offset,
            Bound::Unbounded => self.as_str().len(),
        };

        self.as_str().get(start..end).map(|_| TextSpan { input: self.input, start: self.start + start, end: self.start + end })
    }

    /// Returns the `Span`'s start byte position as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.span(&end);
    ///
    /// assert_eq!(span.start(), 0);
    /// ```
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the `Span`'s end byte position as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.span(&end);
    ///
    /// assert_eq!(span.end(), 0);
    /// ```
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the `Span`'s start `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.clone().span(&end);
    ///
    /// assert_eq!(span.start_pos(), start);
    /// ```
    #[inline]
    pub fn start_pos(&self) -> position::Position<'i> {
        // Span's start position is always a UTF-8 border.
        unsafe { position::Position::new_unchecked(self.input, self.start) }
    }

    /// Returns the `Span`'s end `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.span(&end);
    ///
    /// assert_eq!(span.end_pos(), end);
    /// ```
    #[inline]
    pub fn end_pos(&self) -> position::Position<'i> {
        // Span's end position is always a UTF-8 border.
        unsafe { position::Position::new_unchecked(self.input, self.end) }
    }

    /// Splits the `Span` into a pair of `Position`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.clone().span(&end);
    ///
    /// assert_eq!(span.split(), (start, end));
    /// ```
    #[inline]
    pub fn split(self) -> (position::Position<'i>, position::Position<'i>) {
        // Span's start and end positions are always a UTF-8 borders.
        let pos1 = unsafe { position::Position::new_unchecked(self.input, self.start) };
        let pos2 = unsafe { position::Position::new_unchecked(self.input, self.end) };

        (pos1, pos2)
    }

    /// Captures a slice from the `&str` defined by the `Span`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "abc";
    /// let mut state: Box<yggdrasil_rt::State<'_, Rule>> =
    ///     yggdrasil_rt::State::new(input).skip(1).unwrap();
    /// let start_pos = state.position().clone();
    /// state = state.match_string("b").unwrap();
    /// let span = start_pos.span(&state.position().clone());
    /// assert_eq!(span.as_str(), "b");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'i str {
        // Span's start and end positions are always a UTF-8 borders.
        &self.input[self.start..self.end]
    }

    /// Returns the input string of the `Span`.
    ///
    /// This function returns the input string of the `Span` as a `&str`. This is the source string
    /// from which the `Span` was created. The returned `&str` can be used to examine the contents of
    /// the `Span` or to perform further processing on the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt;
    /// # use yggdrasil_rt::TextSpan;
    ///
    /// // Example: Get input string from a span
    /// let input = "abc\ndef\nghi";
    /// let span = TextSpan::new(input, 1, 7).unwrap();
    /// assert_eq!(span.get_input(), input);
    /// ```
    pub fn get_input(&self) -> &'i str {
        self.input
    }

    /// Iterates over all lines (partially) covered by this span. Yielding a `&str` for each line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "a\nb\nc";
    /// let mut state: Box<yggdrasil_rt::State<'_, Rule>> =
    ///     yggdrasil_rt::State::new(input).skip(2).unwrap();
    /// let start_pos = state.position().clone();
    /// state = state.match_string("b\nc").unwrap();
    /// let span = start_pos.span(&state.position().clone());
    /// assert_eq!(span.lines().collect::<Vec<_>>(), vec!["b\n", "c"]);
    /// ```
    #[inline]
    pub fn lines(&self) -> Lines<'_> {
        Lines { inner: self.lines_span() }
    }

    /// Iterates over all lines (partially) covered by this span. Yielding a `Span` for each line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt;
    /// # use yggdrasil_rt::TextSpan;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "a\nb\nc";
    /// let mut state: Box<yggdrasil_rt::State<'_, Rule>> =
    ///     yggdrasil_rt::State::new(input).skip(2).unwrap();
    /// let start_pos = state.position().clone();
    /// state = state.match_string("b\nc").unwrap();
    /// let span = start_pos.span(&state.position().clone());
    /// assert_eq!(
    ///     span.lines_span().collect::<Vec<_>>(),
    ///     vec![TextSpan::new(input, 2, 4).unwrap(), TextSpan::new(input, 4, 5).unwrap()]
    /// );
    /// ```
    pub fn lines_span(&self) -> LinesSpan<'_> {
        LinesSpan { span: self, pos: self.start }
    }
}

impl<'i> fmt::Debug for TextSpan<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span").field("str", &self.as_str()).field("start", &self.start).field("end", &self.end).finish()
    }
}

impl<'i> PartialEq for TextSpan<'i> {
    fn eq(&self, other: &TextSpan<'i>) -> bool {
        ptr::eq(self.input, other.input) && self.start == other.start && self.end == other.end
    }
}

impl<'i> Eq for TextSpan<'i> {}

impl<'i> Hash for TextSpan<'i> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.input as *const str).hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

/// Merges two spans into one.
///
/// This function merges two spans that are contiguous or overlapping into a single span
/// that covers the entire range of the two input spans. This is useful when you want to
/// aggregate information from multiple spans into a single entity.
///
/// The function checks if the input spans are overlapping or contiguous by comparing their
/// start and end positions. If they are, a new span is created with the minimum start position
/// and the maximum end position of the two input spans.
///
/// If the input spans are neither overlapping nor contiguous, the function returns None,
/// indicating that a merge operation was not possible.
///
/// # Examples
///
/// ```
/// # use yggdrasil_rt;
/// # use yggdrasil_rt::TextSpan;
/// # use yggdrasil_rt::merge_spans;
///
/// // Example 1: Contiguous spans
/// let input = "abc\ndef\nghi";
/// let span1 = TextSpan::new(input, 1, 7).unwrap();
/// let span2 = TextSpan::new(input, 7, 11).unwrap();
/// let merged = merge_spans(&span1, &span2).unwrap();
/// assert_eq!(merged, TextSpan::new(input, 1, 11).unwrap());
///
/// // Example 2: Overlapping spans
/// let input = "abc\ndef\nghi";
/// let span1 = TextSpan::new(input, 1, 7).unwrap();
/// let span2 = TextSpan::new(input, 5, 11).unwrap();
/// let merged = merge_spans(&span1, &span2).unwrap();
/// assert_eq!(merged, TextSpan::new(input, 1, 11).unwrap());
///
/// // Example 3: Non-contiguous spans
/// let input = "abc\ndef\nghi";
/// let span1 = TextSpan::new(input, 1, 7).unwrap();
/// let span2 = TextSpan::new(input, 8, 11).unwrap();
/// let merged = merge_spans(&span1, &span2);
/// assert!(merged.is_none());
/// ```
pub fn merge_spans<'i>(a: &TextSpan<'i>, b: &TextSpan<'i>) -> Option<TextSpan<'i>> {
    if a.end() >= b.start() && a.start() <= b.end() {
        // The spans overlap or are contiguous, so they can be merged.
        TextSpan::new(a.get_input(), core::cmp::min(a.start(), b.start()), core::cmp::max(a.end(), b.end()))
    }
    else {
        // The spans don't overlap and aren't contiguous, so they can't be merged.
        None
    }
}

/// Line iterator for Spans, created by [`Span::lines_span()`].
///
/// Iterates all lines that are at least _partially_ covered by the span. Yielding a `Span` for each.
///
/// [`Span::lines_span()`]: struct.Span.html#method.lines_span
pub struct LinesSpan<'i> {
    span: &'i TextSpan<'i>,
    pos: usize,
}

impl<'i> Iterator for LinesSpan<'i> {
    type Item = TextSpan<'i>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.span.end {
            return None;
        }
        let pos = position::Position::new(self.span.input, self.pos)?;
        if pos.at_end() {
            return None;
        }

        let line_start = pos.find_line_start();
        self.pos = pos.find_line_end();

        TextSpan::new(self.span.input, line_start, self.pos)
    }
}

/// Line iterator for Spans, created by [`Span::lines()`].
///
/// Iterates all lines that are at least _partially_ covered by the span. Yielding a `&str` for each.
///
/// [`Span::lines()`]: struct.Span.html#method.lines
pub struct Lines<'i> {
    inner: LinesSpan<'i>,
}

impl<'i> Iterator for Lines<'i> {
    type Item = &'i str;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|span| span.as_str())
    }
}
