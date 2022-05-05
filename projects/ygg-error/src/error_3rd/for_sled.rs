use diagnostic::DiagnosticLevel;
use sled::{CompareAndSwapError, Error};

use crate::{IOError, RuntimeError, YggdrasilError};

impl From<Error> for YggdrasilError {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(o) => IOError::from(o).as_error(DiagnosticLevel::Error),
            _ => RuntimeError { message: error.to_string() }.as_error(DiagnosticLevel::Error),
        }
    }
}

impl From<CompareAndSwapError> for YggdrasilError {
    fn from(error: CompareAndSwapError) -> Self {
        RuntimeError { message: error.to_string() }.as_error(DiagnosticLevel::Error)
    }
}
