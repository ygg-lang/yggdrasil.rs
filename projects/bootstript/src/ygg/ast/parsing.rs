use super::*;
use tree_sitter::{Parser, Tree, TreeCursor};
use tree_sitter_yg::language;

use lsp_types::Diagnostic;

pub struct MyVisitor {
    warns: Vec<Diagnostic>,
}

pub struct GSTBuilder {
    parser: Parser,
    tree: Tree,
}

impl GSTBuilder {
    // visitor: impl MetaVisitor<MetaData = M> + 'static
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(language())?;
        // test if parser can work
        let tree = match parser.parse("", None) {
            None => {
                panic!("parser init failed")
            }
            Some(s) => s,
        };
        Ok(Self { parser, tree })
    }
    fn update_by_text(&mut self, text: impl AsRef<[u8]>) -> Result<()> {
        //let tree = self.parser.parse(text, Some(&self.tree));
        match self.parser.parse(text, None) {
            Some(s) => self.tree = s,
            None => { panic!("fail to update") }
        }
        Ok(())
    }
}


impl GSTBuilder {
    /// BFS traverse
    pub fn traverse(&mut self) -> Result<Program> {
        let cursor = &mut self.tree.walk();
        Ok(Program::parse(cursor).unwrap().unwrap())
    }
}

impl Program {
    pub fn parse(cursor: &mut TreeCursor) -> Result<Option<Self>> {
        let mut children = vec![];
        let this = cursor.node();
        let range = this.range();
        // visit all children
        cursor.goto_first_child();
        Statement::parse(cursor)?.map(|e|children.push(e));
        for _ in 0..this.child_count() {
            cursor.goto_next_sibling();
            Statement::parse(cursor)?.map(|e|children.push(e));
        }
        // build data
        Ok(Some(Self { children, range }))
    }
}

impl Statement {
    pub fn parse(cursor: &mut TreeCursor) -> Result<Option<Self>> {
        let node = cursor.node();
        let kind = SyntaxKind::from(node);
        let out = match kind {
            SyntaxKind::sym_program => {None}
            SyntaxKind::sym_whitespace => {None}
            SyntaxKind::sym_fragment_statement => {
                FragmentStatement::parse(cursor)?.map(|e|Statement::FragmentStatement(Box::new(e)))
            }
            _ => unimplemented!("{:#?}", kind)
        };
        Ok(out)
    }
}

impl FragmentStatement {
    pub fn parse(cursor: &mut TreeCursor) -> Result<Option<Self>> {
        let this = cursor.node();

        let id = this.child_by_field_name("id").unwrap();
        let out = Self {
            id: Identifier::parse(id)?,
            range: this.range()
        };
        Ok(Some(out))
    }
}

impl Identifier {
    pub fn parse(node: Node) -> Result<Self> {
        let out = Self {
            data: "".to_string(),
            range: node.range()
        };
        Ok(out)
    }
}

const TEST: &str = r#"
grammar! basic;

grammar! basic {}

fragment! basic;
"#;

#[test]
fn main() -> Result<()> {
    let mut parser = GSTBuilder::new()?;
    parser.update_by_text(TEST)?;
    parser.traverse()?;
    Ok(())
}
