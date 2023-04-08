use crate::optimize::{InsertIgnore, RefineRules, RemarkTags};
use yggdrasil_error::{Validate, Validation, YggdrasilError};
use yggdrasil_ir::{grammar::GrammarInfo, traits::CodeOptimizer};
use yggdrasil_parser::YggdrasilANTLR;

pub fn parse_grammar_raw(grammar: &str) -> Result<GrammarInfo, YggdrasilError> {
    Ok(YggdrasilANTLR::parse(grammar)?)
}

pub fn parse_grammar(grammar: &str) -> Validation<GrammarInfo> {
    let mut errors = vec![];
    let mut info = YggdrasilANTLR::parse(grammar).validate(&mut errors)?;
    info = RefineRules::default().optimize(&info).validate(&mut errors)?;
    info = InsertIgnore::default().optimize(&info).validate(&mut errors)?;
    info = RemarkTags::default().optimize(&info).validate(&mut errors)?;
    Validation::Success { value: info, diagnostics: errors }
}
