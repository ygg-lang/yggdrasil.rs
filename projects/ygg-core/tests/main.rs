#[test]
fn ready() {
    println!("it, works!")
}

// #[test]
// fn test_bootstrap() {
//     let input = include_str!("prog.ygg");
//     let mut info = YggdrasilANTLR::parse(input).expect("fail");
//     info = InsertIgnore::default().optimize(&info).unwrap();
//     info = RefineRules::default().optimize(&info).unwrap();
//     let out = info.generate(RustCodegen::default()).unwrap();
//     out.save("../ygg-rt/tests/json5").unwrap()
// }
