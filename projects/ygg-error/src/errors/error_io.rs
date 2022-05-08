use super::*;
use diagnostic::Diagnostic;

// noinspection RsSelfConvention
impl IOError {
    pub fn as_error(self, level: DiagnosticLevel) -> YggdrasilError {
        YggdrasilError { error: Box::new(YggdrasilErrorKind::ErrorIO(self)), level }
    }
    pub fn as_report(&self, level: DiagnosticLevel) -> Diagnostic {
        Diagnostic::new(level)
    }
}

impl From<std::io::Error> for IOError {
    fn from(error: std::io::Error) -> Self {
        IOError { message: error.to_string(), file: Default::default() }
    }
}
