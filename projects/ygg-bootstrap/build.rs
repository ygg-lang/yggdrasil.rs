use peginator::buildscript::Compile;
use std::env::current_dir;
use std::ops::Range;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(current_dir().unwrap());
    let output = path.join("src/parser/ygg.rs");
    Compile::file("ygg.ebnf").destination(output).format().run().unwrap();
}
