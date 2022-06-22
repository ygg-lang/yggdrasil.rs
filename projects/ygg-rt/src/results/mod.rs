use std::{
    convert::Infallible,
    error::Error,
    fmt::{Display, Formatter},
    ops::{ControlFlow, FromResidual, Range, Try},
};

use crate::YState;

mod residual;

mod reason;
mod s_result;

/// Represent a parsed value
pub type Parsed<'i, T> = (YState<'i>, T);

/// Represent as parsing result
pub enum SResult<'i, T> {
    /// The parsing is not finished yet
    Pending(YState<'i>, T),
    /// The parsing is finished, and give the reason why
    Stop(StopBecause),
}

/// Must copy
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
