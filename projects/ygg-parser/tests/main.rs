use std::{fs::File, io::Write};
use yggdrasil_parser::YggdrasilANTLR;

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
