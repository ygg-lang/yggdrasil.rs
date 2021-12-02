use std::env::current_dir;
use std::path::PathBuf;

use peginator::buildscript::Compile;

use yggdrasil_bootstrap::as_peg;
use yggdrasil_shared::Result;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn codegen() -> Result<()> {
    let path = PathBuf::from(current_dir()?);
    let output = path.join("src/ygg.rs");
    Compile::file("src/ygg.ebnf").destination(output).format().run().unwrap();
    Ok(())
}

#[test]
fn regen() {
    codegen().unwrap();
    println!("{}", as_peg(r#"
    def token string IDENTIFIER {
        'a'
    }
    "#))
}