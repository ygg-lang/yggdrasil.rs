use diagnostic::DiagnosticLevel;
use ucd_trie::Error;

use crate::{errors::RuntimeError, YggdrasilError};

impl From<Error> for YggdrasilError {
    fn from(e: Error) -> Self {
        RuntimeError { message: e.to_string() }.as_error(DiagnosticLevel::Error)
    }
}
