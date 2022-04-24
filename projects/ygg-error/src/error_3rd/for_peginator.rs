use std::ops::Range;

use peginator::ParseError;

use crate::{YggdrasilError, YggdrasilErrorKind};

impl From<ParseError> for YggdrasilError {
    fn from(e: ParseError) -> Self {
        YggdrasilError {
            kind: Box::new(YggdrasilErrorKind::LanguageError(e.specifics.to_string())),
            file: None,
            range: Some(Range { start: e.position, end: e.position }),
        }
    }
}
