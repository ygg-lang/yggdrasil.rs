use tree_sitter::{Parser, Tree};

use tree_sitter_yg::language;

mod gen;
mod lst;

pub fn traverse(tree: &Tree) {
    let cursor = &mut tree.walk();
    println!("in!");
    println!("{:#?}", cursor.node());
    let mut recurse = true;
    loop {
        if recurse && cursor.goto_first_child() {
            recurse = true;
        }
        else {
            println!("out!");
            if cursor.goto_next_sibling() {
                println!("in!");
                println!("{:#?}", cursor.node());
                recurse = true;
            }
            else if cursor.goto_parent() {
                recurse = false;
            }
            else {
                break;
            }
        }
    }
}

const TEST: &str = r#"
grammar! basic;

grammar! basic {}

fragment! basic;
"#;

fn main() {
    let mut parser = Parser::new();
    parser.set_language(language()).expect("Fail to load language");
    let parsed = parser.parse(TEST, None).unwrap();
    // let mut ptr  = parsed.walk();
    //
    // let this = ptr.node().descendant_for_byte_range();
    // match this.kind() {
    //     "Program" => {
    //         print!("{}", this.kind_id());
    //     },
    //     _ => unreachable!()
    // }
    traverse(&parsed)
}
