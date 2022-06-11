mod from_ast;
use std::ops::Range;

use yggdrasil_ir::{GrammarInfo, GrammarRule, QError, Validation};
use yggdrasil_parser::{parse_program, ClassStatementNode, ProgramNode, StatementNode};

pub struct ParseContext {
    out: GrammarInfo,
    errors: Vec<QError>,
}

pub fn parse_grammar(input: &str) -> Validation<GrammarInfo> {
    let mut parser = ParseContext { out: GrammarInfo::default(), errors: Vec::new() };
    let out = parse_program(input);
    match out {
        Ok(out) => {
            parser.visit_program(out);
            Validation::Success { value: parser.out, diagnostics: parser.errors }
        }
        Err(e) => {
            Validation::Failure { fatal: QError::syntax_error(e.to_string()).with_range(&e.range()), diagnostics: vec![] }
        }
    }
}
