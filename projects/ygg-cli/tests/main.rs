use peginator::buildscript::Compile;
use std::{env::current_dir, path::PathBuf};

#[test]
fn ready() {
    println!("it works!")
}

fn main() {
    let path = PathBuf::from(current_dir().unwrap());
    let output = path.join("src/parser/ast.rs");
    Compile::file("ygg.ebnf").destination(output).format().run().unwrap();
}
