use std::path::Path;
use yggdrasil_shared::{codegen::BuildRust, FileCache};

pub fn build<I, O>(input: I, output: O, config: BuildRust) -> std::io::Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let mut cache = FileCache::default();
    let file = cache.load_local(input)?;
    config.generate(file, &mut cache, output).option(|e| {
        e.as_report().eprint(&cache).ok();
    });
    Ok(())
}
