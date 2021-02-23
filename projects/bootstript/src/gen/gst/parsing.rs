use super::*;
use tree_sitter::{Parser, Tree};
use tree_sitter_yg::language;

use lsp_types::Diagnostic;

pub struct ExtraData {}

pub struct MyVisitor {
    warns: Vec<Diagnostic>,
}

impl GSTVisitor for MyVisitor {
    type MetaData = ExtraData;

    fn visit_program(&mut self, node: &Node) -> Result<Program<Self::MetaData>> {
        todo!()
    }

    fn visit_grammar_statement(&mut self, node: &Node) -> Result<GrammarStatement<Self::MetaData>> {
        todo!()
    }

    fn visit_eos(&mut self, node: &Node) -> Result<Eos> {
        todo!()
    }
}

pub struct GSTBuilder<M = ()> {
    parser: Parser,
    visitor: Box<dyn GSTVisitor<MetaData = M>>,
    tree: Tree,
}

impl<M> GSTBuilder<M> {
    pub fn new(visitor: impl GSTVisitor<MetaData = M> + 'static) -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(language())?;
        // test if parser can work
        let tree = match parser.parse("", None) {
            None => {
                panic!("parser init failed")
            }
            Some(s) => s,
        };
        Ok(Self { parser, visitor: Box::new(visitor), tree })
    }
    fn update_by_text(&mut self, text: impl AsRef<[u8]>) -> Result<()> {
        match self.parser.parse(text, Some(&self.tree)) {
            Some(s) => self.tree = s,
            None => {}
        }
        Ok(())
    }
    fn traverse(&self) -> Result<()> {
        let cursor = &mut self.tree.walk();
        self.visitor.enter_node(&cursor.node())?;
        let mut recurse = true;
        loop {
            if recurse && cursor.goto_first_child() {
                recurse = self.visitor.enter_node(&cursor.node())?;
            }
            else {
                self.visitor.leave_node(&cursor.node())?;
                if cursor.goto_next_sibling() {
                    recurse = self.visitor.enter_node(&cursor.node())?;
                }
                else if cursor.goto_parent() {
                    recurse = false;
                }
                else {
                    break;
                }
            }
        }
        Ok(())
    }
}

const TEST: &str = r#"
grammar! basic;

grammar! basic {}

fragment! basic;
"#;

#[test]
fn main() -> Result<()> {
    let visitor = MyVisitor { warns: vec![] };
    let mut parser = GSTBuilder::new(visitor)?;
    parser.update_by_text(TEST)?;
    parser.traverse()
}
