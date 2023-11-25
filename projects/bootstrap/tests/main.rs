use bootstrap::build;
use std::path::Path;
use url::Url;
use yggdrasil_shared::{codegen::BuildRust, Failure, FileCache, Success};

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
