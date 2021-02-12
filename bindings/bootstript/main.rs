use tree_sitter_yg::{yggdrasil};
use tree_sitter::Parser;

fn main() {

    let code = r#"
    grammar json;
"#;
    let mut parser = Parser::new();
    parser.set_language(yggdrasil()).expect("Fail to load language");
    let parsed = parser.parse(code, None);

    println!("{:#?}", parsed)
}