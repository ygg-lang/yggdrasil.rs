use std::fs::read_to_string;
use yggdrasil_shared::codegen::BuildRust;

#[test]
#[ignore]
fn debug() -> std::io::Result<()> {
    let input = read_to_string(r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\grammars\dejavu.ygg"#)?;
    let output = r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\src\dejavu"#;
    let builder = BuildRust::default();
    builder.generate(&input, output).unwrap();
    Ok(())
}

#[test]
#[ignore]
fn debug_v() -> std::io::Result<()> {
    let input = read_to_string(r#"C:\Users\Dell\CLionProjects\valkyrie.rs\projects\valkyrie-parser\grammars\valkyrie.ygg"#)?;
    let output = r#"C:\Users\Dell\CLionProjects\valkyrie.rs\projects\valkyrie-parser\src\codegen"#;
    let builder = BuildRust::default();
    builder.generate(&input, output).unwrap();
    Ok(())
}

#[test]
#[ignore]
fn debug_re0() -> std::io::Result<()> {
    let input = read_to_string(r#"C:\Users\Dell\CLionProjects\re0-script\projects\re0-vm\grammars\re0.ygg"#)?;
    let output = r#"C:\Users\Dell\CLionProjects\re0-script\projects\re0-vm\src\codegen"#;
    let builder = BuildRust::default();
    builder.generate(&input, output).unwrap();
    Ok(())
}
