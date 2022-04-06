use std::fs::File;

use yggdrasil_bootstrap::{codegen::Railroad, parser::GrammarParser};
use yggdrasil_error::YggdrasilError;
use yggdrasil_ir::{DeadCodeEliminator, GrammarInfo};

#[test]
fn ready() {
    println!("it, works!")
}

const TEST: &'static str = include_str!("prog.ygg");

#[test]
fn dumper() {
    let info1 = GrammarParser::parse(TEST).unwrap().success;
    dump_railroad(&info1, "tests/test1.svg").unwrap();
    let dce = DeadCodeEliminator::default();
    let info2 = info1.optimize(vec![dce]).unwrap().success;
    dump_railroad(&info2, "tests/test2.svg").unwrap();
}

fn dump_railroad(info: &GrammarInfo, path: &str) -> Result<(), YggdrasilError> {
    let railroad = Railroad::default();
    let diag1 = info.generate(railroad)?.success;
    let mut file = File::create(path)?;
    diag1.write(&mut file)?;
    Ok(())
}
