use bootstrap::build;
use std::path::Path;
use yggdrasil_shared::{
    codegen::{BuildRust, BuildWasi},
    FileCache,
};

#[test]
#[ignore]
fn main() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = here.join("grammars/bootstrap.ygg").canonicalize()?;
    let output = here.join("../ygg-parser/src/bootstrap").canonicalize()?;
    let builder = BuildRust { range_type: "usize".to_string(), ..Default::default() };
    build(input, output, builder)
}

#[test]
#[ignore]
fn debug_v() -> std::io::Result<()> {
    let input = r#"C:\Users\Dell\CLionProjects\valkyrie.rs\projects\valkyrie-parser\grammars\valkyrie.ygg"#;
    let output = r#"C:\Users\Dell\CLionProjects\valkyrie.rs\projects\valkyrie-parser\src\codegen"#;
    let builder = BuildRust { range_type: "u32".to_string(), ..Default::default() };
    build(input, output, builder)
}

#[test]
#[ignore]
fn debug_json5() -> std::io::Result<()> {
    let input = r#"C:\Users\Dell\CLionProjects\serde-json5\projects\json5-parser\grammars\json5.ygg"#;
    let output = r#"C:\Users\Dell\CLionProjects\serde-json5\projects\json5-parser"#;
    let builder = BuildWasi { ..Default::default() };
    let mut cache = FileCache::default();
    let file = cache.load_local(input)?;
    builder.generate(file, &mut cache, output).option(|e| {
        e.as_report().eprint(&cache).ok();
    });
    Ok(())
}

#[test]
#[ignore]
fn debug_dj() -> std::io::Result<()> {
    let input = r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\grammars\dejavu.ygg"#;
    let output = r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\src\dejavu"#;
    let builder = BuildRust::default();
    build(input, output, builder)
}
#[test]
#[ignore]
fn debug_re0() -> std::io::Result<()> {
    let input = r#"C:\Users\Dell\CLionProjects\re0-script\projects\re0-vm\grammars\re0.ygg"#;
    let output = r#"C:\Users\Dell\CLionProjects\re0-script\projects\re0-vm\src\codegen"#;
    let builder = BuildRust::default();
    build(input, output, builder)
}

#[test]
#[ignore]
fn debug_wolfram() -> std::io::Result<()> {
    let input = r#"C:\Users\Dell\IdeaProjects\wolfram-parser\projects\wolfram-parser\grammars\wolfram.ygg"#;
    let output = r#"C:\Users\Dell\IdeaProjects\wolfram-parser\projects\wolfram-parser\src\parser\codegen"#;
    let builder = BuildRust::default();
    build(input, output, builder)
}
