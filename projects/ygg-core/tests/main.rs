use std::fs::read_to_string;
use yggdrasil_shared::codegen::BuildRust;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn run_json5() {
    let builder = BuildRust::default();
    builder
        .generate(
            &read_to_string(r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\grammars\dejavu.ygg"#).unwrap(),
            r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\src\dejavu\"#,
        )
        .unwrap();
}
