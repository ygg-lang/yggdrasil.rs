use crate::optimize::{InsertIgnore, RefineRules, RemarkTags};
use std::str::FromStr;
use yggdrasil_error::{Validate, Validation, YggdrasilError};
use yggdrasil_ir::{grammar::GrammarInfo, traits::CodeOptimizer};

pub fn parse_grammar_raw(grammar: &str) -> Result<GrammarInfo, YggdrasilError> {
    Ok(GrammarInfo::from_str(grammar)?)
}

pub fn parse_grammar(grammar: &str) -> Validation<GrammarInfo> {
    let mut errors = vec![];
    let mut info = GrammarInfo::from_str(grammar).validate(&mut errors)?;
    info = RefineRules::default().optimize(&info).validate(&mut errors)?;
    info = InsertIgnore::default().optimize(&info).validate(&mut errors)?;
    info = RemarkTags::default().optimize(&info).validate(&mut errors)?;
    Validation::Success { value: info, diagnostics: errors }
}
