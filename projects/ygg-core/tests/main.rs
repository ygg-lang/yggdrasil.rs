// #![feature(once_cell)]
// #![feature(allow_fail)]
//
// use lsp_types::Url;
// use std::{fmt::Write, lazy::SyncLazy, str::FromStr};
// use yggdrasil_core::{
//     frontend::{GrammarContext, Translator},
//     manager::YggParser,
//     Result,
// };
//
// pub mod backends;
// pub mod basic;
// pub mod codegen;
// pub mod diagnostic;
// pub mod optimize;
//
// pub static EXAMPLE_URL: SyncLazy<Url> = SyncLazy::new(|| Url::from_str("file://example/path").unwrap());
//
#[test]
fn ready() {
    println!("ready!")
}
//
// pub fn assert_ast(text: &str, target: &str) -> Result<()> {
//     let mut parser = YggParser::default();
//     let mut out = String::new();
//     for stmt in parser.parse_program(text)?.statement {
//         writeln!(out, "{:#?}", stmt)?
//     }
//     assert_eq!(out, target);
//     Ok(())
// }
//
// pub fn assert_diagnostic(text: &str, target: &str) -> Result<()> {
//     let mut ctx = GrammarContext::new(text, &EXAMPLE_URL);
//     let mut parser = YggParser::default();
//     parser.parse_program(text)?.translate(&mut ctx)?;
//     assert_eq!(format!("{:#?}", ctx.get_hints()), target);
//     Ok(())
// }
//
// pub fn assert_optimize(text: &str, target: &str) -> Result<()> {
//     let mut ctx = GrammarContext::new(text, &EXAMPLE_URL);
//     let mut parser = YggParser::default();
//     let mut grammar = parser.parse_program(text)?.translate(&mut ctx)?;
//
//     grammar.optimize_local()?;
//     let mut out = String::new();
//     for rule in grammar.named_rules() {
//         writeln!(out, "{:#?}", rule)?
//     }
//     assert_eq!(out, target);
//     Ok(())
// }
//
// pub fn assert_codegen(text: &str, target: &str) -> Result<()> {
//     let mut ctx = GrammarContext::new(text, &EXAMPLE_URL);
//     let mut parser = YggParser::default();
//     let mut grammar = parser.parse_program(text)?.translate(&mut ctx)?;
//     grammar.optimize_local()?;
//     let (gr, ge) = grammar.build_peg();
//     assert_eq!(format!("{:#?}\n{:#?}", gr, ge), target);
//     Ok(())
// }
