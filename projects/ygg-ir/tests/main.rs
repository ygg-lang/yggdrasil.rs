use std::env::current_dir;

use peginator_codegen::Compile;
use url::Url;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn bootstrap() -> std::io::Result<()> {
    let dir = current_dir()?.join("../ygg-parser").canonicalize()?;

    let ygg = dir.join("src/ygg.ebnf");
    println!("Source: {}", Url::from_file_path(&ygg).unwrap());
    let rust = dir.join("src/ygg.rs");
    Compile::file(ygg).destination(rust).format().run().unwrap();
    Ok(())
}
