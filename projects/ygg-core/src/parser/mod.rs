mod from_ast;
use std::ops::Range;

use yggdrasil_ir::{GrammarInfo, QError, Validation};

pub struct ParseContext {
    out: GrammarInfo,
    errors: Vec<QError>,
}

pub fn parse_grammar(input: &str) -> Validation<GrammarInfo> {
    todo!()
}
