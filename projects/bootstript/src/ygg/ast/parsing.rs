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

pub trait Parsed
where
    Self: Sized,
{
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self>;
    fn named_one(state: &mut YGGBuilder, this: Node, field: &str) -> Result<Self> {
        match this.child_by_field_name(field) {
            Some(node) => Ok(Self::parse(state, node)?),
            None => Err(YGGError::node_missing(field, this.range())),
        }
    }
    fn named_many(state: &mut YGGBuilder, this: Node, field: &str) -> Result<Vec<Self>> {
        let mut children = vec![];
        for node in this.children_by_field_name(field, &mut this.walk()) {
            children.push(Parsed::parse(state, node)?)
        }
        return Ok(children);
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
    pub fn traverse(&mut self) -> Result<Program> {
        let tree = self.tree.clone();
        let this = tree.walk().node();
        Program::parse(self, this)
    }
}

impl Parsed for Program {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let statement = Parsed::named_many(state, this, "statement")?;
        Ok(Self { statement, range: this.range() })
    }
}

impl Parsed for Statement {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        for node in this.children(&mut this.walk()) {
            let out = match SyntaxKind::from(&node) {
                SyntaxKind::sym_grammar_statement => Self::GrammarStatement(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_assign_statement => Self::AssignStatement(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_fragment_statement => Self::FragmentStatement(Box::new(Parsed::parse(state, node)?)),
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", SyntaxKind::from(&node)),
            };
            return Ok(out);
        }
        unreachable!()
    }
}

impl Parsed for GrammarStatement {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let id = Parsed::named_one(state, this, "id")?;
        let ext = Parsed::named_many(state, this, "ext")?;
        Ok(Self { id, ext, range: this.range() })
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
        let id = Parsed::named_one(state, this, "id")?;
        let eq = Parsed::named_one(state, this, "eq")?;
        let rhs = Parsed::named_one(state, this, "rhs")?;
        Ok(Self { id, eq, rhs, range: this.range() })
    }
}

impl Parsed for Expression {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        for node in this.children(&mut this.walk()) {
            let out = match SyntaxKind::from(&node) {
                SyntaxKind::sym_data => Self::Data(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_expression => Self::Priority(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_unary_suffix => Self::UnarySuffix(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_unary_prefix => Self::UnaryPrefix(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_concat_expr => Self::ConcatExpression(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_field_expr => Self::FieldExpression(Box::new(Parsed::parse(state, node)?)),
                _ => {
                    println!("{}", node.to_sexp());
                    unimplemented!("SyntaxKind::{:#?}=>{{}}", SyntaxKind::from(&node))
                },
            };
            return Ok(out);
        }
        unreachable!()
    }
}

impl Parsed for ConcatExpression {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let lhs = Expression::named_one(state, this, "lhs")?;
        let o = String::named_one(state, this, "op")?;
        let r = Expression::named_one(state, this, "rhs")?;
        match lhs {
            Expression::ConcatExpression(box Self { base, op, rhs, range: _ }) => {
                let mut new_op = op.clone();
                let mut new_rhs = rhs.clone();
                new_op.push(o);
                new_rhs.push(r);
                Ok(Self {
                    base,
                    op: new_op,
                    rhs: new_rhs,
                    range: this.range()
                })
            }
            _ => {
                Ok(Self {
                    base: lhs,
                    op: vec![o],
                    rhs: vec![r],
                    range: this.range()
                })
            }
        }
    }
}

impl Parsed for FieldExpression {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let lhs = Parsed::named_one(state, this, "lhs")?;
        let op = Parsed::named_one(state, this, "op")?;
        let rhs = Parsed::named_one(state, this, "rhs")?;
        Ok(Self { lhs, op, rhs, range: this.range() })
    }
}

impl Parsed for UnarySuffix {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let suffix = Parsed::named_one(state, this, "suffix")?;
        let base = Parsed::named_one(state, this, "base")?;
        Ok(Self { suffix, base, range: this.range() })
    }
}

impl Parsed for UnaryPrefix {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let prefix = Parsed::named_one(state, this, "prefix")?;
        let base = Parsed::named_one(state, this, "base")?;
        Ok(Self { prefix, base, range: this.range() })
    }
}

impl Parsed for Data {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        for node in this.children(&mut this.walk()) {
            let out = match SyntaxKind::from(&node) {
                SyntaxKind::sym_id => Self::Identifier(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_unsigned => Self::Integer(Box::new(Parsed::parse(state, node)?)),
                _ => unimplemented!("SyntaxKind::{:#?}=>{{}}", SyntaxKind::from(&node)),
            };
            return Ok(out);
        }
        unreachable!()
    }
}

parsed_wrap!(Identifier => [data]);
parsed_wrap!(Unsigned => [data]);

impl Parsed for (String, Expression) {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        let op = Parsed::named_one(state, this, "op")?;
        let expr = Parsed::named_one(state, this, "expr")?;
        Ok((op, expr))
    }
}

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
test = e1?
test = e1 ~ e2 ~ e3 ~ e4
"#;

const TEST2: &str = r#"
test = e1 ~ e2 ~ e3 ~ e4
"#;

#[test]
fn main() -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(TEST2)?;
    let out = parser.traverse()?;
    println!("{:#?}", out);
    Ok(())
}
