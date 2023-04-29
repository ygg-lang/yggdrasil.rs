use std::{fs::File, io::Write, str::FromStr};
use yggdrasil_parser::{
    bootstrap::{BootstrapParser, BootstrapRule, RootNode},
    YggdrasilANTLR,
};
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_common() -> std::io::Result<()> {
    let input = include_str!("common.ygg");
    let info = YggdrasilANTLR::parse(input).expect("fail");
    let mut file = File::create("tests/common.ron")?;
    file.write_all(format!("{:#?}", info).as_bytes())
}

#[test]
fn test_json5() -> std::io::Result<()> {
    let input = include_str!("json5.ygg");
    let info = YggdrasilANTLR::parse(input).expect("fail");
    let mut file = File::create("tests/json5.ron")?;
    file.write_all(format!("{:#?}", info).as_bytes())
}

#[test]
fn test_unicode() {
    let cst = BootstrapParser::parse_cst(include_str!("json5.ygg"), BootstrapRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_str(include_str!("json5.ygg")).unwrap();
    let mut file = File::create("tests/test_control/test_if.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}
