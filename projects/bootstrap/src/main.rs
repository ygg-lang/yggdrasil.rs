use bootstrap::build;
use std::path::Path;
use yggdrasil_shared::codegen::BuildRust;

fn main() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = here.join("grammars/bootstrap.ygg").canonicalize()?;
    let output = here.join("../ygg-parser/src/antlr").canonicalize()?;
    let builder = BuildRust { range_type: "usize".to_string(), ..Default::default() };
    build(input, output, builder)
}
