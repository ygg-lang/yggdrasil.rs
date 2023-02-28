use yggdrasil_parser::YggdrasilANTLR;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_common() {
    let input = include_str!("common.ygg");
    let info = YggdrasilANTLR::parse(input).expect("fail");
    println!("{:#?}", info)
}
