use std::fs::read_to_string;
use yggdrasil_shared::{codegen::BuildRust, FileCache};

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn run_json5() {
    let builder = BuildRust::default();
    let mut cache = FileCache::default();
    let id = cache.load_local(r#"C:\Users\Dell\CLionProjects\serde-json5\projects\json5-parser\grammars\json5.ygg"#).unwrap();
    builder.generate(id, &mut cache, r#"C:\Users\Dell\CLionProjects\serde-json5\projects\json5-parser\src\codegen"#).unwrap();
}
