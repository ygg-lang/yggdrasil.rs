use std::path::Path;
use url::Url;
use yggdrasil_shared::{codegen::BuildRust, Failure, FileCache, Success};

fn main() -> std::io::Result<()> {
    let mut cache = FileCache::default();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = here.join("grammars/bootstrap.ygg").canonicalize()?;
    let output = here.join("../ygg-parser/src/antlr").canonicalize()?;
    let builder = BuildRust { range_type: "usize".to_string(), ..Default::default() };

    let url = Url::from_file_path(&input).unwrap();
    let file = cache.load_local(input)?;
    unsafe {
        cache.set_source(file, url.to_string());
    }
    match builder.generate(file, &mut cache, output) {
        Success { value: _, diagnostics } => {
            for x in diagnostics {
                x.as_report().print(&cache)?
            }
        }
        Failure { fatal, diagnostics } => {
            fatal.as_report().eprint(&cache)?;
            for x in diagnostics {
                x.as_report().print(&cache)?
            }
        }
    };
    Ok(())
}
