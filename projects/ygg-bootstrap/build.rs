use peginator::buildscript::Compile;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let path = path.ancestors().skip(5).nth(0).ok_or(()).unwrap();
    let output = path.join("projects/ygg-bootstrap/src/ygg.rs");
    Compile::file("ygg.ebnf").destination(output).format().run_exit_on_error();
    println!("cargo:rerun-if-changed=ygg.ebnf")
}

// fn main() {}
