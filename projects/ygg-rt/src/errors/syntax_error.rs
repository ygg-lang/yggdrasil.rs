use crate::{YError, YErrorKind};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
    path::PathBuf,
};

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    file: PathBuf,
    span: Range<usize>,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<SyntaxError> for YError {
    fn from(value: SyntaxError) -> Self {
        Self { kind: Box::new(YErrorKind::SyntaxError(value)) }
    }
}

impl<'i> From<&'i str> for SyntaxError {
    fn from(value: &'i str) -> Self {
        Self { message: value.to_string(), file: Default::default(), span: Default::default() }
    }
}

impl YError {
    pub fn syntax_error<E>(error: E) -> Self
    where
        E: Into<SyntaxError>,
    {
        error.into().into()
    }
}
impl SyntaxError {
    pub fn new<S: ToString>(message: S) -> Self {
        Self { message: message.to_string(), file: Default::default(), span: Default::default() }
    }
}
