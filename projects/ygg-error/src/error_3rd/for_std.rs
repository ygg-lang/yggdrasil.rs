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
        let e = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseIntError> for YggdrasilError {
    fn from(error: ParseIntError) -> Self {
        let e = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseFloatError> for YggdrasilError {
    fn from(error: ParseFloatError) -> Self {
        let e = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<std::io::Error> for YggdrasilError {
    fn from(error: std::io::Error) -> Self {
        let e = IOError { message: error.to_string(), file: Default::default() };
        e.as_error(DiagnosticLevel::Error)
    }
}

impl From<std::fmt::Error> for YggdrasilError {
    fn from(e: std::fmt::Error) -> Self {
        RuntimeError { message: e.to_string() }.as_error(DiagnosticLevel::Error)
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
        RuntimeError { message: e.to_string() }.as_error(DiagnosticLevel::Error)
    }
}
