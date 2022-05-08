use std::{
    env::VarError,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
    sync::PoisonError,
};

use diagnostic::DiagnosticLevel;

use crate::{
    errors::{IOError, RuntimeError, SyntaxError},
    YggdrasilError,
};

impl From<Utf8Error> for YggdrasilError {
    fn from(error: Utf8Error) -> Self {
        SyntaxError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseIntError> for YggdrasilError {
    fn from(error: ParseIntError) -> Self {
        SyntaxError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseFloatError> for YggdrasilError {
    fn from(error: ParseFloatError) -> Self {
        SyntaxError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<std::io::Error> for YggdrasilError {
    fn from(error: std::io::Error) -> Self {
        IOError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<std::fmt::Error> for YggdrasilError {
    fn from(error: std::fmt::Error) -> Self {
        RuntimeError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl<T> From<PoisonError<T>> for YggdrasilError {
    fn from(error: PoisonError<T>) -> Self {
        RuntimeError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<VarError> for YggdrasilError {
    fn from(error: VarError) -> Self {
        RuntimeError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<()> for YggdrasilError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}
