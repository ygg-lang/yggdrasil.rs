#![feature(once_cell)]

use lsp_types::Url;
use std::{fmt::Write, lazy::SyncLazy, str::FromStr};
use yggdrasil_bootstript::{ast::YGGBuilder, Result};

pub mod basic;
pub mod codegen;
pub mod diagnostic;
pub mod optimize;

pub static EXAMPLE_URL: SyncLazy<Url> = SyncLazy::new(|| Url::from_str("file://example/path").unwrap());

#[test]
fn ready() {
    println!("ready!")
}

pub fn assert_ast(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let mut out = String::new();
    for stmt in parser.traverse()?.statement {
        writeln!(out, "{:#?}", stmt)?
    }
    assert_eq!(out, target);
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

pub fn assert_optimize(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let mut grammar = parser.traverse()?.build_grammar((*EXAMPLE_URL).clone())?.0;
    grammar.optimize_local()?;
    let mut out = String::new();
    for rule in grammar.named_rules() {
        writeln!(out, "{:#?}", rule)?
    }
    assert_eq!(out, target);
    Ok(())
}

pub fn assert_codegen(text: &str, target: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let mut grammar = parser.traverse()?.build_grammar((*EXAMPLE_URL).clone())?.0;
    grammar.optimize_local()?;
    let out = grammar.build_input_grammar();
    assert_eq!(format!("{:#?}", out), target);
    Ok(())
}
