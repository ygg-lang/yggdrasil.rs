use std::env::current_dir;
use std::path::PathBuf;

use peginator::buildscript::Compile;
use peginator::PegParser;

use yggdrasil_bootstrap::parser::Program;
use yggdrasil_shared::Result;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn codegen() -> Result<()> {
    let path = PathBuf::from(current_dir()?);
    let output = path.join("src/parser/ygg.rs");
    Compile::file("src/parser/ygg.ebnf").destination(output).format().run().unwrap();
    Ok(())
}

const TEXT: &'static str = r#"
def token string IDENTIFIER {
    | 'a'*
    | 'b'+
    | label:'c'?
    | 'd'
}
"#;

#[test]
fn regen() {
    codegen().unwrap();
    println!("{:#?}", Program::parse(TEXT).unwrap())
}
