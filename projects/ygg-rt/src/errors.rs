use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{cmp, fmt, mem};

use crate::{position::Position, span::TextSpan, TokenPair, YggdrasilRule};

/// Parse-related error type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct YggdrasilError<R> {
    /// Variant of the error
    pub variant: ErrorKind<R>,
    /// Location within the input string
    pub location: InputLocation,
    /// Line/column within the input string
    pub line_col: LineColLocation,
    path: Option<String>,
    line: String,
    continued_line: Option<String>,
}

/// Different kinds of parsing errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorKind<R> {
    /// Generated parsing error with expected and unexpected `Rule`s
    ParsingError {
        /// Positive attempts
        positives: Vec<R>,
        /// Negative attempts
        negatives: Vec<R>,
    },
    /// Unable to convert given node to ast
    InvalidNode {},
    /// Custom error with a message
    CustomError {
        /// Short explanation
        message: String,
    },
}

/// Where an `Error` has occurred.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputLocation {
    /// `Error` was created by `Error::new_from_pos`
    Pos(usize),
    /// `Error` was created by `Error::new_from_span`
    Span((usize, usize)),
}

/// Line/column where an `Error` has occurred.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LineColLocation {
    /// Line/column pair if `Error` was created by `Error::new_from_pos`
    Pos((usize, usize)),
    /// Line/column pairs if `Error` was created by `Error::new_from_span`
    Span((usize, usize), (usize, usize)),
}

impl From<Position<'_>> for LineColLocation {
    fn from(value: Position<'_>) -> Self {
        Self::Pos(value.line_column())
    }
}

impl From<TextSpan<'_>> for LineColLocation {
    fn from(value: TextSpan<'_>) -> Self {
        let (start, end) = value.split();
        Self::Span(start.line_column(), end.line_column())
    }
}

impl<R: YggdrasilRule> YggdrasilError<R> {
    /// Creates `Error` from `ErrorVariant` and `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::error::{YggdrasilError, ErrorKind};
    /// # use yggdrasil_rt::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// let error = YggdrasilError::new_from_offset(
    ///     ErrorKind::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     pos,
    /// );
    ///
    /// println!("{}", error);
    /// ```
    pub fn new_from_offset(variant: ErrorKind<R>, pos: Position<'_>) -> YggdrasilError<R> {
        let visualize_ws = pos.match_char('\n') || pos.match_char('\r');
        let line_of = pos.line_of();
        let line = if visualize_ws { visualize_whitespace(line_of) } else { line_of.replace(&['\r', '\n'][..], "") };
        Self {
            variant,
            location: InputLocation::Pos(pos.offset()),
            path: None,
            line,
            continued_line: None,
            line_col: LineColLocation::Pos(pos.line_column()),
        }
    }

    /// Creates `Error` from `ErrorVariant` and `Span`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::error::{YggdrasilError, ErrorKind};
    /// # use yggdrasil_rt::{Position, TextSpan};
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let start = Position::from_start(input);
    /// # let end = start.clone();
    /// # let span = start.span(&end);
    /// let error = YggdrasilError::new_from_span(
    ///     ErrorKind::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     span,
    /// );
    ///
    /// println!("{}", error);
    /// ```
    pub fn new_from_span(variant: ErrorKind<R>, span: TextSpan<'_>) -> YggdrasilError<R> {
        let end = span.end_pos();
        let mut end_line_col = end.line_column();
        // end position is after a \n, so we want to point to the visual lf symbol
        if end_line_col.1 == 1 {
            let mut visual_end = end;
            visual_end.skip_back(1);
            let lc = visual_end.line_column();
            end_line_col = (lc.0, lc.1 + 1);
        };

        let mut line_iter = span.lines();
        let sl = line_iter.next().unwrap_or("");
        let mut chars = span.as_str().chars();
        let visualize_ws = matches!(chars.next(), Some('\n') | Some('\r')) || matches!(chars.last(), Some('\n') | Some('\r'));
        let start_line = if visualize_ws { visualize_whitespace(sl) } else { sl.to_owned().replace(&['\r', '\n'][..], "") };
        let ll = line_iter.last();
        let continued_line = if visualize_ws { ll.map(str::to_owned) } else { ll.map(visualize_whitespace) };

        Self {
            variant,
            location: InputLocation::Span((span.start(), end.offset())),
            path: None,
            line: start_line,
            continued_line,
            line_col: LineColLocation::Span(span.start_pos().line_column(), end_line_col),
        }
    }

    /// unable to create node
    pub fn invalid_node(pair: TokenPair<R>) -> YggdrasilError<R> {
        Self::new_from_span(ErrorKind::InvalidNode {}, pair.as_span())
    }

    /// Returns `Error` variant with `path` which is shown when formatted with `Display`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::{YggdrasilError};
    /// # use yggdrasil_rt::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// YggdrasilError::new_from_offset(
    ///     ErrorKind::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     pos,
    /// )
    /// .with_path("file.rs");
    /// ```
    pub fn with_path(mut self, path: &str) -> YggdrasilError<R> {
        self.path = Some(path.to_owned());

        self
    }

    /// Returns the path set using [`YggdrasilError::with_path()`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::error::{YggdrasilError, ErrorKind};
    /// # use yggdrasil_rt::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// # let error = YggdrasilError::new_from_offset(
    /// #     ErrorKind::ParsingError {
    /// #         positives: vec![Rule::open_paren],
    /// #         negatives: vec![Rule::closed_paren]
    /// #     },
    /// #     pos);
    /// let error = error.with_path("file.rs");
    /// assert_eq!(Some("file.rs"), error.path());
    /// ```
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Returns the line that the error is on.
    pub fn line(&self) -> &str {
        self.line.as_str()
    }

    /// Renames all `Rule`s if this is a [`ParsingError`]. It does nothing when called on a
    /// [`CustomError`].
    ///
    /// Useful in order to rename verbose rules or have detailed per-`Rule` formatting.
    ///
    /// [`ParsingError`]: enum.ErrorVariant.html#variant.ParsingError
    /// [`CustomError`]: enum.ErrorVariant.html#variant.CustomError
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::error::{YggdrasilError, ErrorKind};
    /// # use yggdrasil_rt::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// YggdrasilError::new_from_offset(
    ///     ErrorKind::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     pos,
    /// )
    /// .renamed_rules(|rule| match *rule {
    ///     Rule::open_paren => "(".to_owned(),
    ///     Rule::closed_paren => "closed paren".to_owned(),
    /// });
    /// ```
    pub fn renamed_rules<F>(mut self, f: F) -> YggdrasilError<R>
    where
        F: FnMut(&R) -> String,
    {
        let variant = match self.variant {
            ErrorKind::ParsingError { positives, negatives } => {
                let message = YggdrasilError::parsing_error_message(&positives, &negatives, f);
                ErrorKind::CustomError { message }
            }
            variant => variant,
        };

        self.variant = variant;

        self
    }

    fn start(&self) -> (usize, usize) {
        match self.line_col {
            LineColLocation::Pos(line_col) => line_col,
            LineColLocation::Span(start_line_col, _) => start_line_col,
        }
    }

    fn spacing(&self) -> String {
        let line = match self.line_col {
            LineColLocation::Pos((line, _)) => line,
            LineColLocation::Span((start_line, _), (end_line, _)) => cmp::max(start_line, end_line),
        };

        let line_str_len = format!("{}", line).len();

        let mut spacing = String::new();
        for _ in 0..line_str_len {
            spacing.push(' ');
        }

        spacing
    }

    fn underline(&self) -> String {
        let mut underline = String::new();

        let mut start = self.start().1;
        let end = match self.line_col {
            LineColLocation::Span(_, (_, mut end)) => {
                let inverted_cols = start > end;
                if inverted_cols {
                    mem::swap(&mut start, &mut end);
                    start -= 1;
                    end += 1;
                }

                Some(end)
            }
            _ => None,
        };
        let offset = start - 1;
        let line_chars = self.line.chars();

        for c in line_chars.take(offset) {
            match c {
                '\t' => underline.push('\t'),
                _ => underline.push(' '),
            }
        }

        if let Some(end) = end {
            underline.push('^');
            if end - start > 1 {
                for _ in 2..(end - start) {
                    underline.push('-');
                }
                underline.push('^');
            }
        }
        else {
            underline.push_str("^---")
        }

        underline
    }

    fn message(&self) -> String {
        self.variant.message().to_string()
    }

    fn parsing_error_message<F>(positives: &[R], negatives: &[R], mut f: F) -> String
    where
        F: FnMut(&R) -> String,
    {
        match (negatives.is_empty(), positives.is_empty()) {
            (false, false) => {
                format!(
                    "unexpected {}; expected {}",
                    YggdrasilError::enumerate(negatives, &mut f),
                    YggdrasilError::enumerate(positives, &mut f)
                )
            }
            (false, true) => format!("unexpected {}", YggdrasilError::enumerate(negatives, &mut f)),
            (true, false) => format!("expected {}", YggdrasilError::enumerate(positives, &mut f)),
            (true, true) => "unknown parsing error".to_owned(),
        }
    }

    fn enumerate<F>(rules: &[R], f: &mut F) -> String
    where
        F: FnMut(&R) -> String,
    {
        match rules.len() {
            1 => f(&rules[0]),
            2 => format!("{} or {}", f(&rules[0]), f(&rules[1])),
            l => {
                let non_separated = f(&rules[l - 1]);
                let separated = rules.iter().take(l - 1).map(f).collect::<Vec<_>>().join(", ");
                format!("{}, or {}", separated, non_separated)
            }
        }
    }

    pub(crate) fn format(&self) -> String {
        let spacing = self.spacing();
        let path = self.path.as_ref().map(|path| format!("{}:", path)).unwrap_or_default();

        let pair = (self.line_col.clone(), &self.continued_line);
        if let (LineColLocation::Span(_, end), Some(ref continued_line)) = pair {
            let has_line_gap = end.0 - self.start().0 > 1;
            if has_line_gap {
                format!(
                    "{s    }--> {p}{ls}:{c}\n\
                     {s    } |\n\
                     {ls:w$} | {line}\n\
                     {s    } | ...\n\
                     {le:w$} | {continued_line}\n\
                     {s    } | {underline}\n\
                     {s    } |\n\
                     {s    } = {message}",
                    s = spacing,
                    w = spacing.len(),
                    p = path,
                    ls = self.start().0,
                    le = end.0,
                    c = self.start().1,
                    line = self.line,
                    continued_line = continued_line,
                    underline = self.underline(),
                    message = self.message()
                )
            }
            else {
                format!(
                    "{s    }--> {p}{ls}:{c}\n\
                     {s    } |\n\
                     {ls:w$} | {line}\n\
                     {le:w$} | {continued_line}\n\
                     {s    } | {underline}\n\
                     {s    } |\n\
                     {s    } = {message}",
                    s = spacing,
                    w = spacing.len(),
                    p = path,
                    ls = self.start().0,
                    le = end.0,
                    c = self.start().1,
                    line = self.line,
                    continued_line = continued_line,
                    underline = self.underline(),
                    message = self.message()
                )
            }
        }
        else {
            format!(
                "{s}--> {p}{l}:{c}\n\
                 {s} |\n\
                 {l} | {line}\n\
                 {s} | {underline}\n\
                 {s} |\n\
                 {s} = {message}",
                s = spacing,
                p = path,
                l = self.start().0,
                c = self.start().1,
                line = self.line,
                underline = self.underline(),
                message = self.message()
            )
        }
    }
}

impl<R: YggdrasilRule> ErrorKind<R> {
    /// Returns the error message for [`ErrorVariant`]
    ///
    /// If [`ErrorVariant`] is [`CustomError`], it returns a
    /// [`Cow::Borrowed`] reference to [`message`]. If [`ErrorVariant`] is [`ParsingError`], a
    /// [`Cow::Owned`] containing "expected [ErrorVariant::ParsingError::positives] [ErrorVariant::ParsingError::negatives]" is returned.
    ///
    /// [`ErrorVariant`]: enum.ErrorVariant.html
    /// [`CustomError`]: enum.ErrorVariant.html#variant.CustomError
    /// [`ParsingError`]: enum.ErrorVariant.html#variant.ParsingError
    /// [`Cow::Owned`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Owned
    /// [`Cow::Borrowed`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Borrowed
    /// [`message`]: enum.ErrorVariant.html#variant.CustomError.field.message
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::error::ErrorKind;
    /// let variant = ErrorKind::<()>::CustomError {
    ///     message: String::from("unexpected error")
    /// };
    ///
    /// println!("{}", variant.message());
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            ErrorKind::ParsingError { ref positives, ref negatives } => {
                Cow::Owned(YggdrasilError::parsing_error_message(positives, negatives, |r| format!("{:?}", r)))
            }
            ErrorKind::CustomError { ref message } => Cow::Borrowed(message),
            ErrorKind::InvalidNode { .. } => {
                todo!()
            }
        }
    }
}

impl<R: YggdrasilRule> fmt::Display for YggdrasilError<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl<R: YggdrasilRule> fmt::Display for ErrorKind<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::ParsingError { .. } => write!(f, "parsing error: {}", self.message()),
            ErrorKind::CustomError { .. } => write!(f, "{}", self.message()),
            ErrorKind::InvalidNode { .. } => {
                todo!()
            }
        }
    }
}

fn visualize_whitespace(input: &str) -> String {
    input.to_owned().replace('\r', "␍").replace('\n', "␊")
}

#[cfg(test)]
mod tests {
    use super::{super::position, *};
    use alloc::vec;

    #[test]
    fn display_parsing_error_mixed() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> = YggdrasilError::new_from_offset(
            ErrorKind::ParsingError { positives: vec![1, 2, 3], negatives: vec![4, 5, 6] },
            pos,
        );

        assert_eq!(
            format!("{}", error),
            [" --> 2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = unexpected 4, 5, or 6; expected 1, 2, or 3"].join("\n")
        );
    }

    #[test]
    fn display_parsing_error_positives() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_offset(ErrorKind::ParsingError { positives: vec![1, 2], negatives: vec![] }, pos);

        assert_eq!(format!("{}", error), [" --> 2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = expected 1 or 2"].join("\n"));
    }

    #[test]
    fn display_parsing_error_negatives() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_offset(ErrorKind::ParsingError { positives: vec![], negatives: vec![4, 5, 6] }, pos);

        assert_eq!(
            format!("{}", error),
            [" --> 2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = unexpected 4, 5, or 6"].join("\n")
        );
    }

    #[test]
    fn display_parsing_error_unknown() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_offset(ErrorKind::ParsingError { positives: vec![], negatives: vec![] }, pos);

        assert_eq!(
            format!("{}", error),
            [" --> 2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = unknown parsing error"].join("\n")
        );
    }

    #[test]
    fn display_custom_pos() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_offset(ErrorKind::CustomError { message: "error: big one".to_owned() }, pos);

        assert_eq!(format!("{}", error), [" --> 2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = error: big one"].join("\n"));
    }

    #[test]
    fn display_custom_span_two_lines() {
        let input = "ab\ncd\nefgh";
        let start = position::Position::new(input, 4).unwrap();
        let end = position::Position::new(input, 9).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_span(ErrorKind::CustomError { message: "error: big one".to_owned() }, start.span(&end));

        assert_eq!(
            format!("{}", error),
            [" --> 2:2", "  |", "2 | cd", "3 | efgh", "  |  ^^", "  |", "  = error: big one"].join("\n")
        );
    }

    #[test]
    fn display_custom_span_three_lines() {
        let input = "ab\ncd\nefgh";
        let start = position::Position::new(input, 1).unwrap();
        let end = position::Position::new(input, 9).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_span(ErrorKind::CustomError { message: "error: big one".to_owned() }, start.span(&end));

        assert_eq!(
            format!("{}", error),
            [" --> 1:2", "  |", "1 | ab", "  | ...", "3 | efgh", "  |  ^^", "  |", "  = error: big one"].join("\n")
        );
    }

    #[test]
    fn display_custom_span_two_lines_inverted_cols() {
        let input = "abcdef\ngh";
        let start = position::Position::new(input, 5).unwrap();
        let end = position::Position::new(input, 8).unwrap();
        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_span(ErrorKind::CustomError { message: "error: big one".to_owned() }, start.span(&end));

        assert_eq!(
            format!("{}", error),
            [" --> 1:6", "  |", "1 | abcdef", "2 | gh", "  | ^----^", "  |", "  = error: big one"].join("\n")
        );
    }

    #[test]
    fn display_custom_span_end_after_newline() {
        let input = "abcdef\n";
        let start = position::Position::new(input, 0).unwrap();
        let end = position::Position::new(input, 7).unwrap();
        assert!(start.at_start());
        assert!(end.at_end());

        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_span(ErrorKind::CustomError { message: "error: big one".to_owned() }, start.span(&end));

        assert_eq!(
            format!("{}", error),
            [" --> 1:1", "  |", "1 | abcdef␊", "  | ^-----^", "  |", "  = error: big one"].join("\n")
        );
    }

    #[test]
    fn display_custom_span_empty() {
        let input = "";
        let start = position::Position::new(input, 0).unwrap();
        let end = position::Position::new(input, 0).unwrap();
        assert!(start.at_start());
        assert!(end.at_end());

        let error: YggdrasilError<u32> =
            YggdrasilError::new_from_span(ErrorKind::CustomError { message: "error: empty".to_owned() }, start.span(&end));

        assert_eq!(format!("{}", error), [" --> 1:1", "  |", "1 | ", "  | ^", "  |", "  = error: empty"].join("\n"));
    }

    #[test]
    fn mapped_parsing_error() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> = YggdrasilError::new_from_offset(
            ErrorKind::ParsingError { positives: vec![1, 2, 3], negatives: vec![4, 5, 6] },
            pos,
        )
        .renamed_rules(|n| format!("{}", n + 1));

        assert_eq!(
            format!("{}", error),
            [" --> 2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = unexpected 5, 6, or 7; expected 2, 3, or 4"].join("\n")
        );
    }

    #[test]
    fn error_with_path() {
        let input = "ab\ncd\nef";
        let pos = position::Position::new(input, 4).unwrap();
        let error: YggdrasilError<u32> = YggdrasilError::new_from_offset(
            ErrorKind::ParsingError { positives: vec![1, 2, 3], negatives: vec![4, 5, 6] },
            pos,
        )
        .with_path("file.rs");

        assert_eq!(
            format!("{}", error),
            [" --> file.rs:2:2", "  |", "2 | cd", "  |  ^---", "  |", "  = unexpected 4, 5, or 6; expected 1, 2, or 3"]
                .join("\n")
        );
    }

    #[test]
    fn underline_with_tabs() {
        let input = "a\txbc";
        let pos = position::Position::new(input, 2).unwrap();
        let error: YggdrasilError<u32> = YggdrasilError::new_from_offset(
            ErrorKind::ParsingError { positives: vec![1, 2, 3], negatives: vec![4, 5, 6] },
            pos,
        )
        .with_path("file.rs");

        assert_eq!(
            format!("{}", error),
            [" --> file.rs:1:3", "  |", "1 | a	xbc", "  |  	^---", "  |", "  = unexpected 4, 5, or 6; expected 1, 2, or 3"]
                .join("\n")
        );
    }

    #[test]
    fn pos_to_lcl_conversion() {
        let input = "input";

        let pos = Position::new(input, 2).unwrap();

        assert_eq!(LineColLocation::Pos(pos.line_column()), pos.into());
    }

    #[test]
    fn span_to_lcl_conversion() {
        let input = "input";

        let span = TextSpan::new(input, 2, 4).unwrap();
        let (start, end) = span.split();

        assert_eq!(LineColLocation::Span(start.line_column(), end.line_column()), span.into());
    }
}
