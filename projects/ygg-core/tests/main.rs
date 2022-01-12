use yggdrasil_core::{codegen::as_peg, frontend::GrammarInfo};

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
// @export
// @position
// Program = {statements:Statement} $;
//
// Statement =
// @:DefineStatement [EOS]
// | @:EmptyStatement
// ;
//
// EmptyStatement = EOS;
//
// @char
// EOS = ';';
//
// @position
// DefineStatement = define:DEFINE modifiers:Modifiers symbol:Identifier [arguments:Arguments] [type:Typing] '{' body:Choice '}';
//
// @string
// @no_skip_ws
// DEFINE = 'def!' | 'def';
//
// @position
// Modifiers = {id:Identifier !('{'|'('|':'|'->'|';')};
//
// @position
// Choice = [['|'] terms:Term {"|" terms:Term}];
//
// @position
// Term = {prefix:Prefix} [tag:Identifier ':'] node:Node {suffix:Suffix};
//
// @position
// Arguments = '(' ')';
//
// @position
// Typing = ('->' | ':') id:Identifier;
//
// @char
// Prefix = '^' | '!';
//
// @char
// Suffix = '+' | '*' | '?';
//
// @position
// Node =
// @:Group |
// @:Charset |
// @:StringLiteral |
// @:Identifier
// ;
// @position
// Group = "(" body:Choice ")";
//
// @position
// @no_skip_ws
// StringLiteral =
// '"' {!'"' body:StringItem } '"' |
// "'" {!"'" body:StringItem } "'"
// ;
//
// @no_skip_ws
// StringItem =
// @:StringEscaped |
// @:CharOne
// ;
//
// StringEscaped = '\\' char:char;
//
// @position
// Charset = '[' {CharItem} ']';
//
// CharItem =
// @:CharRange |
// @:CharOne
// ;
// @position
// CharRange = start:char '-' end:char;
// @char
// CharOne = char;
//
// @position
// @string
// @no_skip_ws
// Identifier = (XID_START | '_') {XID_CONTINUE};
//
//
// @char
// @check(unicode_xid::UnicodeXID::is_xid_start)
// XID_START = char;
//
//
// @char
// @check(unicode_xid::UnicodeXID::is_xid_continue)
// XID_CONTINUE = char;

const INPUT: &'static str = r#"
def atomic Identifier -> string {
    (XID_START | '_')
}
"#;

#[test]
fn test() {
    let grammar = GrammarInfo::parse(INPUT).unwrap();
    println!("{}", as_peg(&grammar));
}

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
