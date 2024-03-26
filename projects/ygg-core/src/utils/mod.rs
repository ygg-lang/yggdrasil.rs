use crate::{
    optimize::{InlineRules, InsertIgnore, RefineRules, RemarkTags},
    FileCache,
};
use yggdrasil_error::{FileID, Validate, Validation, YggdrasilError};
use yggdrasil_ir::{grammar::GrammarInfo, traits::CodeOptimizer};
use yggdrasil_parser::parse_grammar_info;

pub fn parse_grammar_raw(grammar: &str) -> Result<GrammarInfo, YggdrasilError> {
    let mut cache = FileCache::default();
    let id = cache.load_text(grammar, "x");
    parse_grammar_info(&mut cache, id).result(|_| {})
}

pub fn parse_grammar(id: FileID, cache: &mut FileCache) -> Validation<GrammarInfo> {
    let mut errors = vec![];
    let mut info = parse_grammar_info(cache, id).validate(&mut errors)?;
    info = InsertIgnore::default().optimize(&info).validate(&mut errors)?;
    info = InlineRules::default().optimize(&info).validate(&mut errors)?;
    info = RefineRules::default().optimize(&info).validate(&mut errors)?;
    info = RemarkTags::default().optimize(&info).validate(&mut errors)?;
    Validation::Success { value: info, diagnostics: errors }
}
