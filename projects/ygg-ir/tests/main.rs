use std::path::PathBuf;

use peginator_codegen::Compile;
use url::Url;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn bootstrap_old() -> std::io::Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../ygg-parser").canonicalize()?;
    let ygg = dir.join("src/ygg.ebnf");
    println!("Source: {}", Url::from_file_path(&ygg).unwrap());
    let rust = dir.join("src/ygg.rs");
    println!("Target: {}", Url::from_file_path(&rust).unwrap());
    Compile::file(ygg).destination(rust).format().run().unwrap();
    Ok(())
}
