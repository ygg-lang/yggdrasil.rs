use diagnostic::{DiagnosticLevel, Span};
use peginator::ParseError;

use crate::{errors::SyntaxError, YggdrasilError};

impl From<ParseError> for YggdrasilError {
    fn from(error: ParseError) -> Self {
        let e = SyntaxError {
            message: error.specifics.to_string(),
            file: Default::default(),
            span: Span { start: error.position, end: error.position },
        };
        e.as_error(DiagnosticLevel::Error)
    }
}
