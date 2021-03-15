use super::*;
use tree_sitter::{Parser, Tree, TreeCursor};
use tree_sitter_yg::language;

use lsp_types::Diagnostic;
use std::borrow::Borrow;

macro_rules! parsed_wrap {
    ($t:ty => [$i:ident]) => {
        impl Parsed for $t {
            fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
                let $i = Parsed::parse(state, this)?;
                Ok(Self { $i, range: this.range() })
            }
        }
    };
}

pub trait Parsed where Self: Sized {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self>;
    fn named_one(state: &mut YGGBuilder, this: Node, field: &str) -> Result<Self> {
        match this.child_by_field_name(field) {
            Some(node) => Ok(Self::parse(state, node)?),
            None => Err(YGGError::node_missing(field, this.range())),
        }
    }
}

pub struct YGGBuilder {
    parser: Parser,
    tree: Tree,
    text: String,
    warns: Vec<Diagnostic>,
}

impl YGGBuilder {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(language())?;
        // test if parser can work
        let tree = parser.parse("", None).ok_or(YGGError::init_fail())?;
        Ok(Self { parser, tree, text: String::new(), warns: vec![] })
    }
    fn update_by_text(&mut self, text: &str) -> Result<()> {
        // let tree = self.parser.parse(text, Some(&self.tree));
        match self.parser.parse(text.as_bytes(), None) {
            Some(s) => {
                self.text = String::from(text);
                self.tree = s;
                self.warns = vec![];
            }
            None => {
                panic!("fail to update")
            }
        }
        Ok(())
    }
}

impl YGGBuilder {
    pub fn traverse(&mut self) -> Result<Program> {
        let tree = self.tree.clone();
        let this = tree.walk().node();
        Program::parse(self, this)
    }
}

impl Parsed for Program {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let mut children = vec![];
        for node in this.children(&mut this.walk()) {
            let kind = SyntaxKind::from(&node);
            match kind {
                SyntaxKind::sym_WHITESPACE => {
                    continue;
                }
                SyntaxKind::sym_statement => children.push(Parsed::parse(state, node)?),
                SyntaxKind::sym_eos => {
                    println!("{:#?}", kind)
                }
                // SyntaxKind::sym_program=>{
                //     println!("{:#?}", kind)
                // }
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
            }
        }
        Ok(Self { children, range: this.range() })
    }
}

impl Parsed for Statement {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        for node in this.children(&mut this.walk()) {
            let kind = SyntaxKind::from(&node);
            let out = match kind {
                SyntaxKind::sym_grammar_statement => {
                    Statement::GrammarStatement(Box::new(Parsed::parse(state, node)?))
                }
                SyntaxKind::sym_assign_statement => {
                    Statement::AssignStatement(Box::new(Parsed::parse(state, node)?))
                }
                SyntaxKind::sym_fragment_statement => {
                    Statement::FragmentStatement(Box::new(Parsed::parse(state, node)?))
                }
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
            };
            return Ok(out);
        }
        unreachable!()
    }
}

impl Parsed for GrammarStatement {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let id = Identifier::named_one(state, this, "id")?;
        Ok(Self { id, ext: vec![], range: this.range() })
    }
}

impl Parsed for FragmentStatement {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let id = Identifier::named_one(state, this, "id")?;
        Ok(Self { id, range: this.range() })
    }
}

impl Parsed for AssignStatement {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let id = Identifier::named_one(state, this, "id")?;
        let eq = String::named_one(state, this, "eq")?;
        let rhs = Expression::named_one(state, this, "rhs")?;
        Ok(Self { id, eq, rhs, range: this.range() })
    }
}

impl Parsed for Expression {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        for node in this.children(&mut this.walk()) {
            let kind = SyntaxKind::from(&node);
            let out = match kind {
                SyntaxKind::sym_expression => {
                    let out = Expression::parse(state, node)?;
                    Self::Priority(Box::new(out))
                }
                SyntaxKind::sym_id => {
                    let out = Identifier::parse(state, node)?;
                    Self::Identifier(Box::new(out))
                }
                SyntaxKind::sym_unsigned => {
                    let out = Unsigned::parse(state, node)?;
                    Self::Integer(Box::new(out))
                }
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", kind),
            };
            return Ok(out);
        }
        unreachable!()
    }
}

parsed_wrap!(Identifier => [data]);
parsed_wrap!(Unsigned => [data]);

impl Parsed for usize {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        Ok(this.utf8_text(state.text.as_bytes())?.parse::<usize>()?)
    }
}

impl Parsed for String {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        Ok(this.utf8_text(state.text.as_bytes())?.to_string())
    }
}

const TEST: &str = r#"
fragment! a;
"#;

#[test]
fn main() -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(TEST)?;
    let out = parser.traverse()?;
    println!("{:#?}", out);
    Ok(())
}
