use crate::{yggdrasil};
use tree_sitter::Parser;


const TEST:&str = r#"
fragment! yg;
"#;


#[test]
fn main() {
    let mut parser = Parser::new();
    parser.set_language(yggdrasil()).expect("Fail to load language");
    let parsed = parser.parse(TEST, None).unwrap();
    println!("{:#?}", parsed)
}