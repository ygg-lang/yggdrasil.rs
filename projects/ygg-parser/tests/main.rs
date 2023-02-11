use yggdrasil_parser::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_common() {
    let input = include_str!("common.ygg");
    let info = YggdrasilParser::parse(input).expect("fail");
    println!("{:#?}", info)
}
