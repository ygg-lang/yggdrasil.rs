use super::*;
use tree_sitter::{Tree, Parser};
use tree_sitter_yg::yggdrasil;

pub fn traverse(tree: &Tree) {
    let cursor = &mut tree.walk();
    println!("in!");
    println!("{:#?}", cursor.node());
    let mut recurse = true;
    loop {
        if recurse && cursor.goto_first_child() {
            recurse = true;
        } else {
            println!("out!");
            if cursor.goto_next_sibling() {
                println!("in!");
                println!("{:#?}", cursor.node());
                recurse = true;
            } else if cursor.goto_parent() {
                recurse = false;
            } else {
                break;
            }
        }
    }
}


const TEST:&str = r#"
grammar! basic;

grammar! basic {}

fragment! basic;
"#;

pub struct State {
    parser: Parser,
}

impl State {
    pub fn new() {

    }
}

#[test]
fn main() {
    let mut parser = Parser::new();
    parser.set_language(yggdrasil()).expect("Fail to load language");
    let parsed = parser.parse(TEST, None).unwrap();
    traverse(&parsed)
}