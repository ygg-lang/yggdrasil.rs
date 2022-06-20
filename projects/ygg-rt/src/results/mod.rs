use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Range,
};

#[derive(Copy, Clone, Debug)]
pub enum StopBecause {
    Uninitialized,
    MissingEof { position: usize },
    MissingRepeats { min: usize, current: usize, position: usize },
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
            StopBecause::MissingEof { position } => f.write_fmt(format_args!("Missing EOF at position {}", position)),
            StopBecause::MissingRepeats { min, current, position } => {
                f.write_fmt(format_args!("Missing at least {} repeats (got {}) at position {}", min, current, position))
            }
            StopBecause::MissingCharacter { expected, position } => {
                f.write_fmt(format_args!("Missing character '{}' at position {}", expected, position))
            }
            StopBecause::MissingCharacterRange { start, end, position } => {
                f.write_fmt(format_args!("Missing character in range '{}'..='{}' at position {}", start, end, position))
            }
            StopBecause::MissingString { message, position } => {
                f.write_fmt(format_args!("Missing string '{}' at position {}", message, position))
            }
            StopBecause::ShouldNotBe { message, position } => {
                f.write_fmt(format_args!("Should not be '{}' at position {}", message, position))
            }
            StopBecause::Custom { message, position } => {
                f.write_fmt(format_args!("Custom error '{}' at position {}", message, position))
            }
        }
    }
}
impl StopBecause {
    pub fn range(&self) -> Range<usize> {
        match self {
            StopBecause::Uninitialized => 0..0,
            StopBecause::MissingEof { position } => *position..*position,
            StopBecause::MissingRepeats { min: _, current: _, position } => *position..*position,
            StopBecause::MissingCharacter { expected, position } => *position..*position + expected.len_utf8(),
            StopBecause::MissingCharacterRange { start: _, end: _, position } => *position..*position + 1,
            StopBecause::MissingString { message, position } => *position..*position + message.len(),
            StopBecause::ShouldNotBe { message: _, position } => *position..*position + 1,
            StopBecause::Custom { message: _, position } => *position..*position + 1,
        }
    }
}
