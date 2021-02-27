use super::*;
use tree_sitter::{Parser, Tree, TreeCursor};
use tree_sitter_yg::language;

use lsp_types::Diagnostic;

pub struct ExtraData {}

pub struct MyVisitor {
    warns: Vec<Diagnostic>,
}

impl HiddenFilter for MyVisitor {}

impl GSTVisitor for MyVisitor {
    type MetaData = ExtraData;
    // may can be const


    fn visit_program(&mut self, node: &Node) -> Option<Self::MetaData> {
        None
    }

    fn visit_grammar_statement(&mut self, node: &Node) -> Option<Self::MetaData> {
        None
    }

    fn visit_eos(&mut self, node: &Node) -> Option<Self::MetaData> {
        None
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
}

use SyntaxKind::*;

impl<M> GSTBuilder<M> {
    /// BFS traverse
    pub fn traverse(&mut self) -> Result<GSTNode<Program<M>,M>> {
        Program::traverse(&mut self.visitor, &mut self.tree.walk())
    }

}

impl<M> Program<M> {
    pub fn traverse(visitor: &mut Box<dyn GSTVisitor<MetaData = M>>, cursor: &mut TreeCursor) -> Result<GSTNode<Program<M>,M>> {
        let meta = visitor.visit_program(&cursor.node());
        let out = GSTNode::<Program<M>,M> {
            data: Program::visit(cursor)?,
            meta,
        };
        Ok(out)
    }
    pub fn visit(cursor: &mut TreeCursor) -> Result<Self> {
        let mut v = vec![];
        if !cursor.goto_first_child() {
            panic!("no child")
        }
        let node = cursor.node();
        v.push(node);
        unimplemented!()
    }
}

impl<M> AuxNode1<M> {
    pub fn visit(cursor: &mut TreeCursor) -> Result<Self>{
        let mut v = vec![];
        if !cursor.goto_first_child() {
            panic!("no child")
        }
        let node = cursor.node();
        v.push(node);
        let kind = SyntaxKind::node_kind(&cursor.node());
        let out = match kind {
            sym_program => Self::GrammarStatement(unimplemented!()),
            _ => unimplemented!("{:?}", kind)
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
    let visitor = MyVisitor { warns: vec![] };
    let mut parser = GSTBuilder::new(visitor)?;
    parser.update_by_text(TEST)?;
    parser.traverse();
    Ok(())
}
