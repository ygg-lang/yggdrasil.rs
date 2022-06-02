use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone)]
pub enum YError {
    Uninitialized,
    ExceptRepeats { min: usize, current: usize },
    ExceptedEof,
    ExceptedCharacter(char),
    ExceptedCharacterRange(char, char),
    ExceptedString(&'static str),
}

impl Default for YError {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Error for YError {}

impl Display for YError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
