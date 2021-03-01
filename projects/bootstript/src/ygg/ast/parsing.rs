use super::*;
use tree_sitter::{Parser, Tree, TreeCursor};
use tree_sitter_yg::language;

use lsp_types::Diagnostic;
use std::borrow::Borrow;

pub struct MyVisitor {
    warns: Vec<Diagnostic>,
}

pub struct GSTBuilder {
    parser: Parser,
    tree: Tree,
    text: String,
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
        Ok(Self { parser, tree, text: String::new() })
    }
    fn update_by_text(&mut self, text: &str) -> Result<()> {
        // let tree = self.parser.parse(text, Some(&self.tree));
        match self.parser.parse(text.as_bytes(), None) {
            Some(s) => {
                self.text = String::from(text);
                self.tree = s
            }
            None => {
                panic!("fail to update")
            }
        }
        Ok(())
    }
}

impl GSTBuilder {
    pub fn traverse(&mut self) -> Result<Program> {
        let tree = self.tree.clone();
        let this = tree.walk().node();
        self.parse_program(this)
    }

    pub fn parse_program(&mut self, this: Node) -> Result<Program> {
        let mut children = vec![];
        for node in this.children(&mut this.walk()) {
            let kind = SyntaxKind::from(&node);
            match kind {
                SyntaxKind::sym_whitespace => {
                    continue;
                }
                SyntaxKind::sym_statement => children.push(self.parse_statement(node)?),
                SyntaxKind::sym_eos => {
                    println!("{:#?}", kind)
                }
                // SyntaxKind::sym_program=>{
                //     println!("{:#?}", kind)
                // }
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
            }
        }
        Ok(Program { children, range: this.range() })
    }
    fn parse_statement(&mut self, this: Node) -> Result<Statement> {
        let node = match this.child(0) {
            Some(s) => s,
            None => {
                panic!("missing node")
            }
        };
        let kind = SyntaxKind::from(&node);
        let out = match kind {
            SyntaxKind::sym_grammar_statement => {
                let out = self.parse_grammar_statement(node)?;
                Statement::GrammarStatement(Box::new(out))
            }
            SyntaxKind::sym_assign_statement => {
                let out = self.parse_assign_statement(node)?;
                Statement::AssignStatement(Box::new(out))
            }
            SyntaxKind::sym_fragment_statement => {
                let out = self.parse_fragment_statement(node)?;
                Statement::FragmentStatement(Box::new(out))
            }
            _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
        };
        Ok(out)
    }
    fn parse_grammar_statement(&mut self, this: Node) -> Result<GrammarStatement> {
        let mut id = Default::default();
        let mut ext = Default::default();
        for node in this.children(&mut this.walk()) {
            let kind = SyntaxKind::from(&node);
            match kind {
                // Ignored group
                SyntaxKind::sym_whitespace => continue,
                // Uncollected group
                SyntaxKind::sym_grammar | SyntaxKind::sym_eos => continue,
                // Anonymous group
                SyntaxKind::anon_sym_LBRACE | SyntaxKind::anon_sym_RBRACE => continue,
                // Named group
                SyntaxKind::sym_id => id = self.parse_id(node)?,
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
            }
        }
        Ok(GrammarStatement { id, ext, range: this.range() })
    }
    fn parse_fragment_statement(&mut self, this: Node) -> Result<FragmentStatement> {
        let mut id = Default::default();
        for node in this.children(&mut this.walk()) {
            let kind = SyntaxKind::from(&node);
            match kind {
                // Ignored group
                SyntaxKind::sym_whitespace => continue,
                // Uncollected group
                SyntaxKind::sym_fragment | SyntaxKind::sym_eos => continue,
                // Anonymous group
                // Named group
                SyntaxKind::sym_id => id = self.parse_id(node)?,
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
            }
        }
        Ok(FragmentStatement { id, range: this.range() })
    }
    fn parse_assign_statement(&mut self, this: Node) -> Result<AssignStatement> {
        unimplemented!()
    }
    fn parse_id(&mut self, this: Node) -> Result<Identifier> {
        let out = Identifier { data: "".to_string(), range: this.range() };
        Ok(out)
    }
}

impl Statement {
    pub fn parse(cursor: &mut TreeCursor) -> Result<Option<Self>> {
        let node = cursor.node();
        let kind = SyntaxKind::from(node);
        let out = match kind {
            SyntaxKind::sym_program => None,
            SyntaxKind::sym_whitespace => None,
            SyntaxKind::sym_fragment_statement => {
                FragmentStatement::parse(cursor)?.map(|e| Statement::FragmentStatement(Box::new(e)))
            }
            _ => unimplemented!("{:#?}", kind),
        };
        Ok(out)
    }
}

impl FragmentStatement {
    pub fn parse(cursor: &mut TreeCursor) -> Result<Option<Self>> {
        let this = cursor.node();

        let id = this.child_by_field_name("id").unwrap();
        let out = Self { id: Identifier::parse(id)?, range: this.range() };
        Ok(Some(out))
    }
}

impl Identifier {
    pub fn parse(node: Node) -> Result<Self> {
        let out = Self { data: "".to_string(), range: node.range() };
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
