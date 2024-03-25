use std::mem::take;
use yggdrasil_error::{FileID, YggdrasilError};
use yggdrasil_ir::grammar::GrammarInfo;

pub struct ParseContext {
    pub id: FileID,
}

#[derive(Default)]
pub struct ParseState {
    errors: Vec<YggdrasilError>,
    pub grammar: GrammarInfo,
}

impl ParseState {
    pub fn add_error(&mut self, error: YggdrasilError) {
        self.errors.push(error);
    }
    pub fn get_errors(&mut self) -> Vec<YggdrasilError> {
        take(&mut self.errors)
    }
}
