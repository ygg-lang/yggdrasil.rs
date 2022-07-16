use crate::SyntaxError;
use std::{ops::Range, path::PathBuf};

pub mod syntax_error;

pub type YResult<T> = Result<T, YError>;

pub struct YError {
    kind: Box<YErrorKind>,
}

pub enum YErrorKind {
    IoError { message: String, path: String },
    ParseError { message: String },
    SyntaxError(SyntaxError),
    OtherError,
}
