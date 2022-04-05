use std::fs::File;

use yggdrasil_bootstrap::{codegen::Railroad, parser::GrammarParser};
use yggdrasil_ir::DeadCodeEliminator;

#[test]
fn ready() {
    println!("it, works!")
}

const TEST: &'static str = r#"
def Program {
    (A1 | A2) | (B1? | B2+ | B3*) | "string"
}

def Other {
    a | _b
}
"#;

#[test]
fn dumper() {
    let info = GrammarParser::parse(TEST).unwrap().success;
    let railroad = Railroad::default();
    let dce = DeadCodeEliminator::default();
    let diag = info.generate(railroad).unwrap().success;
    let mut file = File::create("tests/test0.svg").unwrap();
    diag.write(&mut file).unwrap();
    let mut file = File::create("tests/test1.svg").unwrap();
    diag.write(&mut file).unwrap();
}
