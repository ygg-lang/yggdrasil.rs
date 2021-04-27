#![feature(once_cell)]

pub mod basic;
pub mod diagnostic;
pub mod input_grammar;

use lsp_types::Url;
use std::{lazy::SyncLazy, str::FromStr};
use yggdrasil_bootstript::{ast::YGGBuilder, Result};

pub static EXAMPLE_URL: SyncLazy<Url> = SyncLazy::new(|| Url::from_str("file://example/path").unwrap());

#[test]
fn ready() {
    println!("ready!")
}

pub fn assert_ast(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let out = parser.traverse()?;
    assert_eq!(format!("{:#?}", out), target);
    Ok(())
}

pub fn assert_diagnostic(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let out = parser.traverse()?;
    let diag = out.build_grammar((*EXAMPLE_URL).clone())?.1;
    assert_eq!(format!("{:#?}", diag), target);
    Ok(())
}
