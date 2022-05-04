use super::*;

// noinspection RsSelfConvention
impl SyntaxError {
    pub fn as_error(self, level: DiagnosticLevel) -> YggdrasilError {
        YggdrasilError { error: Box::new(YggdrasilErrorKind::SyntaxError(self)), level }
    }
    pub fn set_file(&mut self, file: FileID) {
        self.file = file;
    }
    pub fn with_file(mut self, file: FileID) -> Self {
        self.set_file(file);
        self
    }
}
