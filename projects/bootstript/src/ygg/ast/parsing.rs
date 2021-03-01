use super::*;
use tree_sitter::{Parser, Tree, TreeCursor};
use tree_sitter_yg::language;

use lsp_types::Diagnostic;

pub type DynMetaVisitor<M> = Box<dyn MetaVisitor<MetaData = M>>;

pub struct ExtraData {}

pub struct MyVisitor {
    warns: Vec<Diagnostic>,
}

impl MetaVisitor for MyVisitor {
    type MetaData = ExtraData;
    // may can be const

    fn visit_meta(&mut self, _: &Node) -> Option<Self::MetaData> {
        None
    }

    fn visit_program(&mut self, cursor: &mut TreeCursor) -> Result<GSTNode<Program<Self::MetaData>, Self::MetaData>> {
        let mut v = vec![];
        let node = cursor.node();
        let meta = self.visit_meta(&node);
        cursor.goto_first_child();
        for _ in 0..node.child_count() {
            let result = self.visit_aux_node1(cursor);
            cursor.goto_next_sibling();
            v.push(result);
        }
        unimplemented!()
        // let data = Program {
        //     children: v
        // };
        // let out = GSTNode::<Self, M> { data: T::traverse(cursor)?, meta };
        // Ok(out)
    }

    fn visit_aux_node1(&mut self, cursor: &mut TreeCursor) -> Result<GSTNode<AuxNode1<Self::MetaData>, Self::MetaData>> {
        unimplemented!()
    }

    fn visit_grammar_statement(&mut self, _: &Node) -> Option<Self::MetaData> {
        None
    }

    fn visit_eos(&mut self, _: &Node) -> Option<Self::MetaData> {
        None
    }
}

pub struct GSTBuilder<M = ()> {
    parser: Parser,
    visitor: DynMetaVisitor<M>,
    tree: Tree,
}

impl<M> GSTBuilder<M> {
    pub fn new(visitor: impl MetaVisitor<MetaData = M> + 'static) -> Result<Self> {
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
        //let tree = self.parser.parse(text, Some(&self.tree));
        match self.parser.parse(text, None) {
            Some(s) => self.tree = s,
            None => {panic!("fail to update")}
        }
        Ok(())
    }
}

use SyntaxKind::*;

impl<M> GSTBuilder<M> {
    /// BFS traverse
    pub fn traverse(&mut self) -> Result<GSTNode<Program<M>, M>> {
        let cursor = &mut self.tree.walk();
        self.visitor.visit_program(cursor)
    }

}

impl<M> Program<M> {
    pub fn parse(visitor: &mut DynMetaVisitor<M>, cursor: &mut TreeCursor) -> Result<GSTNode<Self, M>> {
        let meta = visitor.visit_meta(&cursor.node());
        let out = GSTNode::<Self, M> { data: Self::traverse(cursor)?, meta };
        Ok(out)
    }
    pub fn traverse( cursor: &mut TreeCursor) -> Result<Self> {

        unimplemented!()
    }
}

#[allow(unreachable_code)]
impl<M> AuxNode1<M> {
    pub fn parse(cursor: &mut TreeCursor) -> Result<Self> {
        let mut v = vec![];

        if !cursor.goto_first_child() {
            panic!("no child AuxNode1")
        }
        let node = cursor.node();
        v.push(node);
        let kind = SyntaxKind::from(cursor.node());
        let out = match kind {
            sym_program => Self::GrammarStatement(unimplemented!()),
            _ => unimplemented!("{:?}", kind),
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
    parser.traverse()?;
    Ok(())
}
