use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Copy, Clone, Debug)]
pub enum StopBecause {
    Uninitialized,
    MissingEof { position: usize },
    MissingRepeats { min: usize, current: usize, position: usize },
    MissingCharacter { expected: char, position: usize },
    MissingCharacterRange { start: char, end: char, position: usize },
    MissingString { message: &'static str, position: usize },
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
        todo!()
    }
}
