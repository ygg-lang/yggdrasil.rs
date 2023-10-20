use std::path::Path;
use yggdrasil_shared::codegen::BuildRust;

fn main() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = include_str!("../grammars/bootstrap.ygg");
    let output = here.join("../ygg-parser/src/antlr").canonicalize()?;
    let builder = BuildRust::default();
    builder.generate(input, output).unwrap();
    Ok(())
}
