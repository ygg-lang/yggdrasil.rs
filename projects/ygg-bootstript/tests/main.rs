pub mod basic;
pub mod diagnostic;

use yggdrasil_bootstript::{ast::YGGBuilder, Result};

#[test]
fn ready() {
    println!("ready!")
}

pub fn assert_ast(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let out = parser.traverse()?;
    assert_eq!(target, format!("{:#?}", out));
    Ok(())
}

pub fn assert_diagnostic(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let out = parser.traverse()?;
    let diag = out.build_grammar(None)?.show_diagnostic();
    assert_eq!(target, format!("{:#?}", diag));
    Ok(())
}
