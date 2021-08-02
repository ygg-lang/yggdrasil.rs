use crate::{
    frontend::{FilePosition, Translator},
    Result,
};
use lsp_types::Url;
use std::str::FromStr;
use yggdrasil_bootstrap::ast::YggParser;

fn test_gen(text: &str) -> Result<String> {
    let url = Url::from_str("file://example/path").unwrap();
    let file = FilePosition::new(text, &url);
    let mut parser = YggParser::default();
    let state = parser.parse_program(text)?.translate(&file)?.0;
    Ok(state.build_peg_code())
}

#[test]
fn test_build() {
    println!("{}", test_gen("a = 0").unwrap())
}
