use std::path::Path;
use url::Url;
use yggdrasil_shared::{codegen::BuildRust, Failure, FileCache, Success};

pub fn build<I, O>(input: I, output: O, config: BuildRust) -> std::io::Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let mut cache = FileCache::default();
    let url = Url::from_file_path(&input).unwrap();
    let file = cache.load_local(input)?;
    unsafe {
        cache.set_source(file, url.to_string());
    }
    match config.generate(file, &mut cache, output) {
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
