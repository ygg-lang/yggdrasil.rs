use std::fs::read_to_string;
use yggdrasil_shared::codegen::RustCodegen;

#[test]
#[ignore]
fn debug() -> std::io::Result<()> {
    let input = read_to_string(r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\grammars\dejavu.ygg"#)?;
    let output = r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\src\dejavu"#;
    let builder = RustCodegen::default();
    builder.generate(&input, output).unwrap();
    Ok(())
}
