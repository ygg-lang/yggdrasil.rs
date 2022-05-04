use diagnostic::DiagnosticLevel;
use url::ParseError;

use crate::{errors::SyntaxError, YggdrasilError};

impl From<ParseError> for YggdrasilError {
    fn from(e: ParseError) -> Self {
        SyntaxError { message: e.to_string(), file: Default::default(), span: Default::default() }
            .as_error(DiagnosticLevel::Error)
    }
}
