use super::*;
use yggdrasil_core::railroad::DEFAULT_CSS;

pub mod railroad;

pub fn assert_codegen(text: &str, target: &str) -> Result<()> {
    let mut ctx = GrammarContext::new(text, &EXAMPLE_URL);
    let mut parser = YggParser::default();
    let grammar = parser.parse_program(text)?.translate(&mut ctx)?;
    let svg = grammar.railroad_svg(DEFAULT_CSS);
    assert_eq!(format!("{}", svg), target);
    Ok(())
}
