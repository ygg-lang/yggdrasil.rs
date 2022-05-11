use peginator_codegen::Compile;
use std::env::current_dir;

fn main() -> std::io::Result<()> {
    let dir = current_dir()?.join("../ygg-parser").canonicalize()?;
    let ygg = dir.join("src/ygg.ebnf");
    let rust = dir.join("src/ygg.rs");
    Compile::file(ygg).destination(rust).format().run().unwrap();
    Ok(())
}
