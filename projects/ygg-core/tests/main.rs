use std::{fs::File, io::Write};
use yggdrasil_core::{
    codegen::RustCodegen,
    optimize::{InsertIgnore, RefineRules},
};
use yggdrasil_ir::traits::CodeOptimizer;
use yggdrasil_parser::YggdrasilParser;

// mod json;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn test_bootstrap() {
    let input = include_str!("prog.ygg");
    let mut info = YggdrasilParser::parse(input).expect("fail");
    info = InsertIgnore::default().optimize(&info).unwrap();
    info = RefineRules::default().optimize(&info).unwrap();
    let out = info.generate(RustCodegen::default());
    let mut output = File::create("../ygg-rt/tests/json5/mod.rs").unwrap();
    output.write_all(out.unwrap().as_bytes()).unwrap();
}

// #[test]
// fn dumper() -> QResult {
//     let info1 = GrammarParser::parse(include_str!("prog.ygg"))?.success;
//     // let mut ast = File::create("tests/test.yaml")?;
//     // ast.write_all(serde_yaml::to_string(&info1).unwrap().as_bytes())?;
//     dump_railroad(&info1, "tests/test1.svg")?;
//     let dce = DeadCodeEliminator::default();
//     let info2 = info1.optimize(vec![dce])?.success;
//     dump_railroad(&info2, "tests/test2.svg")?;
//     Ok(())
// }
//
// #[test]
// fn dumper2() {
//     let mut cg = PegCodegen::default();
//     cg.codegen("tests/prog.ygg").unwrap()
// }
//
// fn dump_railroad(info: &GrammarInfo, path: &str) -> Result<(), YggdrasilError> {
//     let railroad = Railroad::default();
//     let diag1 = info.generate(railroad)?.success;
//     let mut file = File::create(path)?;
//     diag1.write(&mut file)?;
//     Ok(())
// }
