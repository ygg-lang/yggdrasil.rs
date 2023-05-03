use std::{fs::File, io::Write, str::FromStr};
use yggdrasil_parser::bootstrap::{BootstrapParser, BootstrapRule, ClassStatementNode, RootNode};
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_json5() {
    let cst = BootstrapParser::parse_cst(include_str!("json5.ygg"), BootstrapRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_str(include_str!("json5.ygg")).unwrap();
    let mut file = File::create("tests/json5.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_string() {
    let text = r##"class StringRaw -> StringText {
    /[^"]*/
}"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ClassStatementNode::from_str(text).unwrap();
    println!("{ast:#?}")
}
