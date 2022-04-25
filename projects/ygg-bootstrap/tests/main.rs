use std::{fs::File, io::Write};

use yggdrasil_bootstrap::{codegen::Railroad, parser::GrammarParser};
use yggdrasil_error::YggdrasilError;
use yggdrasil_ir::{DeadCodeEliminator, GrammarInfo};

#[test]
fn ready() {
    println!("it, works!")
}

const TEST: &'static str = include_str!("prog.ygg");

#[test]
fn dumper() -> Result<(), YggdrasilError> {
    let info1 = GrammarParser::parse(TEST)?.success;
    let mut ast = File::create("tests/test.yaml")?;
    ast.write_all(serde_yaml::to_string(&info1).unwrap().as_bytes())?;
    dump_railroad(&info1, "tests/test1.svg")?;
    let dce = DeadCodeEliminator::default();
    let info2 = info1.optimize(vec![dce])?.success;
    dump_railroad(&info2, "tests/test2.svg")?;
    Ok(())
}

fn dump_railroad(info: &GrammarInfo, path: &str) -> Result<(), YggdrasilError> {
    let railroad = Railroad::default();
    let diag1 = info.generate(railroad)?.success;
    let mut file = File::create(path)?;
    diag1.write(&mut file)?;
    Ok(())
}
