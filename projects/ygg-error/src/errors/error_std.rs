use super::*;
use std::{
    env::VarError,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
    sync::PoisonError,
};

impl From<Utf8Error> for YggdrasilError {
    fn from(e: Utf8Error) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<ParseIntError> for YggdrasilError {
    fn from(e: ParseIntError) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<ParseFloatError> for YggdrasilError {
    fn from(e: ParseFloatError) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<std::io::Error> for YggdrasilError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: IOError(e), file: None, range: None }
    }
}

impl From<fmt::Error> for YggdrasilError {
    fn from(e: fmt::Error) -> Self {
        Self { kind: FormatError(e), file: None, range: None }
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
