use crate::{position::Position, span::TextSpan, YggdrasilRule};
use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{cmp, fmt, mem};

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
    InvalidNode {
        expect: R,
    },
    InvalidTag {
        expect: Cow<'static, str>,
    },
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
    pub fn invalid_node(expect: R, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(ErrorKind::InvalidNode { expect }, span)
    }
    /// unable to create node
    pub fn missing_tag(expect: Cow<'static, str>, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(ErrorKind::InvalidTag { expect }, span)
    }
    /// missing rule
    pub fn missing_rule(expect: R, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(ErrorKind::InvalidNode { expect }, span)
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
            ErrorKind::InvalidNode { expect } => Cow::Owned(format!("invalid node, expected node {expect:?}")),
            ErrorKind::InvalidTag { .. } => {
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
            ErrorKind::InvalidNode { .. } => {
                write!(f, "{}", self.message())
            }
            ErrorKind::InvalidTag { .. } => {
                todo!()
            }
            ErrorKind::CustomError { .. } => write!(f, "{}", self.message()),
        }
    }
}

fn visualize_whitespace(input: &str) -> String {
    input.to_owned().replace('\r', "␍").replace('\n', "␊")
}
