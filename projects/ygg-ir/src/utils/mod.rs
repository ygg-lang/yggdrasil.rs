use std::mem::take;
use yggdrasil_error::{FileID, YggdrasilError};

pub(crate) struct ParseContext {
    pub id: FileID,
    errors: Vec<YggdrasilError>,
}

impl ParseContext {
    pub fn new(id: FileID) -> Self {
        Self { id, errors: vec![] }
    }
    pub fn add_error(&mut self, error: YggdrasilError) {
        self.errors.push(error);
    }
    pub fn get_errors(&mut self) -> Vec<YggdrasilError> {
        take(&mut self.errors)
    }
}
