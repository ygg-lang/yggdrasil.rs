use std::mem::take;
use yggdrasil_error::{FileID, Result, YggdrasilError};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    rule::{GrammarRule, YggdrasilIdentifier},
};

#[derive(Default)]
pub struct ParseContext {
    pub file: FileID,
}

#[derive(Default)]
pub struct ParseState {
    errors: Vec<YggdrasilError>,
    grammar: GrammarInfo,
}

impl ParseState {
    pub fn add_error(&mut self, error: YggdrasilError) {
        self.errors.push(error);
    }
    pub fn get_errors(&mut self) -> Vec<YggdrasilError> {
        take(&mut self.errors)
    }
    pub fn get_grammar(&mut self) -> GrammarInfo {
        take(&mut self.grammar)
    }

    pub fn rename(&mut self, name: YggdrasilIdentifier) {
        self.grammar.name = name
    }

    pub fn register(&mut self, rule: GrammarRule) -> Result<()> {
        match self.grammar.rules.insert(rule.name.text.clone(), rule) {
            Some(_) => Err(YggdrasilError::runtime_error("dup")),
            None => Ok(()),
        }
    }
}
