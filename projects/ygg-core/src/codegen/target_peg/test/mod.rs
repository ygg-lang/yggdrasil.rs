use crate::{
    frontend::{GrammarContext, Translator},
    Result,
};
use lsp_types::Url;
use std::str::FromStr;
use yggdrasil_bootstrap::ast::YggParser;

fn test_gen(text: &str) -> Result<String> {
    let url = Url::from_str("file://example/path").unwrap();
    let mut ctx = GrammarContext::new(text, &url);
    let mut parser = YggParser::default();
    let state = parser.parse_program(text)?.translate(&mut ctx)?;
    Ok(state.build_peg_code())
}

#[test]
fn test_build() {
    println!("{}", test_gen("a = 0").unwrap())
}
