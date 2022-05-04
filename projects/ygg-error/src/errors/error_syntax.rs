use super::*;
use diagnostic::Diagnostic;

// noinspection RsSelfConvention
impl SyntaxError {
    pub fn as_error(self, level: DiagnosticLevel) -> YggdrasilError {
        YggdrasilError { error: Box::new(YggdrasilErrorKind::ErrorSyntax(self)), level }
    }
    pub fn as_report(&self, level: DiagnosticLevel) -> Diagnostic {
        Diagnostic::new(level)
    }
    pub fn set_file(&mut self, file: FileID) {
        self.file = file;
    }
    pub fn with_file(mut self, file: FileID) -> Self {
        self.set_file(file);
        self
    }
}
