use diagnostic::Diagnostic;

use super::*;

// noinspection RsSelfConvention
impl RuntimeError {
    pub fn as_error(self, level: DiagnosticLevel) -> YggdrasilError {
        YggdrasilError { error: Box::new(YggdrasilErrorKind::ErrorRuntime(self)), level }
    }
    pub fn as_report(&self, level: DiagnosticLevel) -> Diagnostic {
        Diagnostic::new(level)
    }
}

impl<E> From<E> for RuntimeError
where
    E: Error,
{
    fn from(error: E) -> Self {
        RuntimeError { message: error.to_string() }
    }
}
