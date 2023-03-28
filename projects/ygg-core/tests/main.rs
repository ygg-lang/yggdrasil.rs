use std::fs::{read_to_string, File};
use yggdrasil_shared::codegen::RustCodegen;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn run_json5() {
    let builder = RustCodegen::default();
    builder
        .generate(
            &read_to_string(r#"C:\Users\Dell\CLionProjects\yggdrasil-template\projects\build_by_dep\grammars\json5.ygg"#)
                .unwrap(),
            r#"C:\Users\Dell\CLionProjects\yggdrasil-template\projects\build_by_dep\src\json5"#,
        )
        .unwrap();
}
