use super::*;
use std::{
    env::VarError,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
    sync::PoisonError,
};

impl From<Utf8Error> for YggdrasilError {
    fn from(error: Utf8Error) -> Self {
        let e = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseIntError> for YggdrasilError {
    fn from(e: ParseIntError) -> Self {
        let e = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseFloatError> for YggdrasilError {
    fn from(e: ParseFloatError) -> Self {
        let e = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<std::io::Error> for YggdrasilError {
    fn from(e: std::io::Error) -> Self {
        Self { error: Box::new(IOError(e)), level: None, range: None }
    }
}

impl From<std::fmt::Error> for YggdrasilError {
    fn from(e: std::fmt::Error) -> Self {
        Self { error: Box::new(FormatError(e)), level: None, range: None }
    }
}

impl<T> From<PoisonError<T>> for YggdrasilError {
    fn from(_: PoisonError<T>) -> Self {
        Self::unreachable()
    }
}

impl From<()> for YggdrasilError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}

impl From<VarError> for YggdrasilError {
    fn from(e: VarError) -> Self {
        Self::language_error(e.to_string())
    }
}
