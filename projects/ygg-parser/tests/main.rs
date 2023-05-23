use std::{fs::File, io::Write, str::FromStr};

use yggdrasil_parser::bootstrap::{BootstrapParser, BootstrapRule, ClassStatementNode, ExpressionNode, RootNode};
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_preview() {
    let cst = yggdrasil_parser::antlr::BootstrapParser::parse_cst(
        include_str!("bootstrap.ygg"),
        yggdrasil_parser::antlr::BootstrapRule::Root,
    )
    .unwrap();
    println!("Short Form:\n{}", cst);
    let ast = yggdrasil_parser::antlr::RootNode::from_str(include_str!("bootstrap.ygg")).unwrap();
    let mut file = File::create("tests/bootstrap.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_bootstrap() {
    let cst = BootstrapParser::parse_cst(include_str!("bootstrap.ygg"), BootstrapRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_str(include_str!("bootstrap.ygg")).unwrap();
    let mut file = File::create("tests/bootstrap.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_string() {
    let text = r##"class ClassStatement {
    DecoratorCall* ModifierCall* ^KW_CLASS (name:Identifier) ("->" cast:Identifier)? OP_REMARK? ClassBlock
}"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ClassStatementNode::from_str(text).unwrap();
    println!("{ast:#?}")
}

#[test]
fn test_atomic() {
    let text = r##"("->")"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::Expression).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ExpressionNode::from_str(text).unwrap();
    println!("{ast:#?}")
}
