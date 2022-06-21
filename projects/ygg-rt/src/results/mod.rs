use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Range,
};

use crate::ast_mode::YState;

mod y_result;

pub type Parsed<'i, T> = (YState<'i>, T);

/// will change [`SResult`] to [`SResult`] if try 2.0 is stable
pub enum SResult<'i, T> {
    Pending(YState<'i>, T),
    Stop(StopBecause),
}

#[derive(Copy, Clone, Debug)]
pub enum StopBecause {
    Uninitialized,
    ExpectEof { position: usize },
    ExpectRepeats { min: usize, current: usize, position: usize },
    MissingCharacter { expected: char, position: usize },
    MissingCharacterRange { start: char, end: char, position: usize },
    MissingString { message: &'static str, position: usize },
    MustBe { message: &'static str, position: usize },
    ShouldNotBe { message: &'static str, position: usize },
    Custom { message: &'static str, position: usize },
}

impl Default for StopBecause {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Error for StopBecause {}

impl Display for StopBecause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StopBecause::Uninitialized => f.write_str("Uninitialized"),
            StopBecause::ExpectEof { .. } => f.write_str("Expect end of file"),
            StopBecause::ExpectRepeats { min, current, .. } => {
                f.write_fmt(format_args!("Expect at least {} repeats (got {})", min, current))
            }
            StopBecause::MissingCharacter { expected, .. } => f.write_fmt(format_args!("Missing character '{}'", expected)),
            StopBecause::MissingCharacterRange { start, end, .. } => {
                f.write_fmt(format_args!("Expect character in range '{}'..='{}'", start, end))
            }
            StopBecause::MissingString { message, .. } => f.write_fmt(format_args!("Missing string '{}'", message)),
            StopBecause::MustBe { message, .. } => f.write_fmt(format_args!("Must be `{}`", message)),
            StopBecause::ShouldNotBe { message, .. } => f.write_fmt(format_args!("Should not be `{}`", message)),
            StopBecause::Custom { message, .. } => f.write_str(message),
        }
    }
}

impl StopBecause {
    pub fn range(&self) -> Range<usize> {
        match *self {
            StopBecause::Uninitialized => 0..0,
            StopBecause::ExpectEof { position } => position..position + 1,
            StopBecause::ExpectRepeats { min: _, current: _, position } => position..position + 1,
            StopBecause::MissingCharacter { expected, position } => position..position + expected.len_utf8(),
            StopBecause::MissingCharacterRange { start: _, end: _, position } => position..position + 1,
            StopBecause::MissingString { message, position } => position..position + message.len(),
            StopBecause::MustBe { message: _, position } => position..position + 1,
            StopBecause::ShouldNotBe { message: _, position } => position..position + 1,
            StopBecause::Custom { message: _, position } => position..position + 1,
        }
    }
}
