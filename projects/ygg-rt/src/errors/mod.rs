pub use self::source::YggdrasilErrorSource;
use crate::{span::TextSpan, YggdrasilRule};
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{fmt, fmt::Display, ops::Range};

mod source;

/// Parse-related error type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct YggdrasilError<R> {
    /// Variant of the error
    pub variant: YggdrasilErrorKind<R>,
    /// Location within the input string
    pub location: Range<usize>,
    /// The file path where the error occurred
    pub source: YggdrasilErrorSource,
}

/// Different kinds of parsing errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum YggdrasilErrorKind<R> {
    /// Generated parsing error with expected and unexpected `Rule`s
    InvalidRule {
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
        expect: Vec<String>,
    },
    /// Custom error with a message
    CustomError {
        /// Short explanation
        message: String,
    },
}

impl<R: YggdrasilRule> YggdrasilError<R> {
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
    pub fn new_from_span(variant: YggdrasilErrorKind<R>, span: TextSpan<'_>) -> YggdrasilError<R> {
        let end = span.end_pos();
        // let end_line_col = end.line_column();
        // end position is after a \n, so we want to point to the visual lf symbol
        // if end_line_col.1 == 1 {
        //     let mut visual_end = end;
        //     visual_end.skip_back(1);
        // };
        Self { variant, location: span.start()..end.offset(), source: Default::default() }
    }

    /// unable to create node
    pub fn invalid_node(expect: R, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(YggdrasilErrorKind::InvalidNode { expect }, span)
    }
    /// unable to create node
    pub fn missing_tag(expect: &str, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(YggdrasilErrorKind::InvalidTag { expect: vec![expect.to_string()] }, span)
    }
    /// missing rule
    pub fn missing_rule(expect: R, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(YggdrasilErrorKind::InvalidNode { expect }, span)
    }

    /// missing rule
    pub fn custom_error<S: Display>(message: S, span: TextSpan) -> YggdrasilError<R> {
        Self::new_from_span(YggdrasilErrorKind::CustomError { message: message.to_string() }, span)
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
        self.source = YggdrasilErrorSource::Local(path.to_string());
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
    pub fn get_local_path(&self) -> Option<&str> {
        match &self.source {
            YggdrasilErrorSource::Local(v) => Some(v.as_str()),
            _ => None,
        }
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
            YggdrasilErrorKind::InvalidRule { positives, negatives } => {
                let message = YggdrasilError::parsing_error_message(&positives, &negatives, f);
                YggdrasilErrorKind::CustomError { message }
            }
            variant => variant,
        };

        self.variant = variant;

        self
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
}

impl<R: YggdrasilRule> Display for YggdrasilError<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.variant, f)
    }
}

impl<R: YggdrasilRule> Display for YggdrasilErrorKind<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YggdrasilErrorKind::InvalidRule { positives, negatives } => {
                write!(f, "invalid rule, except: {:?}, found: {:?}", positives, negatives)
            }
            YggdrasilErrorKind::InvalidNode { expect } => {
                write!(f, "invalid node, except: {:?}", expect)
            }
            YggdrasilErrorKind::InvalidTag { expect } => {
                write!(f, "invalid tag, except: {:?}", expect)
            }
            YggdrasilErrorKind::CustomError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

#[allow(unused)]
fn visualize_whitespace(input: &str) -> String {
    input.to_owned().replace('\r', "␍").replace('\n', "␊")
}
