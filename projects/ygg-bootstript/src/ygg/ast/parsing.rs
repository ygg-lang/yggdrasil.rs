use super::*;
use tree_sitter::{Parser, Tree, TreeCursor};
use tree_sitter_ygg::language;

use std::{borrow::Borrow, ops::AddAssign};

macro_rules! parsed_wrap {
    ($t:ty: $($i:ident << $method:tt),+) => {
        impl Parsed for $t {
            fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
                $(let $i = parsed_wrap!(@ state, this, $method);)+
                #[cfg(feature="lsp")]
                let range = convert_range(this.range());
                #[cfg(not(feature="lsp"))]
                let range = this.range();
                Ok(Self { $($i,)+ range })
            }
        }
    };
    (@ $state:ident,$this:ident,$method:ident) => {Parsed::$method($state, $this)?};
    (@ $state:ident,$this:ident,($method:ident, $name:literal))=>{Parsed::$method($state, $this, $name)?};
}

pub trait Parsed
where
    Self: Sized,
{
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self>;
    fn named_one(state: &mut YGGBuilder, this: Node, field: &str) -> Result<Self> {
        match this.child_by_field_name(field) {
            Some(node) => Ok(Self::parse(state, node)?),
            None => {
                unreachable!();
                Err(YGGError::node_missing(field, this.range()))
            }
        }
    }
    fn named_some(state: &mut YGGBuilder, this: Node, field: &str) -> Result<Option<Self>> {
        match this.child_by_field_name(field) {
            Some(node) => Ok(Some(Self::parse(state, node)?)),
            None => Ok(None),
        }
    }
    fn named_many(state: &mut YGGBuilder, this: Node, field: &str) -> Result<Vec<Self>> {
        let mut children = vec![];
        // TODO: reduce cursor alloc
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
}

impl YGGBuilder {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(language())?;
        // test if parser can work
        let tree = parser.parse("", None).ok_or(YGGError::init_fail())?;
        Ok(Self { parser, tree, text: String::new() })
    }
    pub fn update_by_text(&mut self, text: &str) -> Result<()> {
        // let tree = self.parser.parse(text, Some(&self.tree));
        match self.parser.parse(text.as_bytes(), None) {
            Some(s) => {
                self.text = String::from(text);
                self.tree = s;
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

parsed_wrap!(Program:
    statement << (named_many, "statement")
);

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

parsed_wrap!(AssignStatement:
    id << (named_one, "id"),
    eq << (named_one, "eq"),
    rhs << (named_one, "rhs")
);

parsed_wrap!(GrammarStatement:
    id << (named_one, "id"),
    ext << (named_many, "ext")
);

parsed_wrap!(FragmentStatement:
    id << (named_one, "id")
);

impl Parsed for Expression {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        for node in this.children(&mut this.walk()) {
            let out = match SyntaxKind::from(&node) {
                SyntaxKind::sym_expression => return Self::parse(state, node),
                SyntaxKind::sym_data => Self::Data(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_unary_suffix => Self::UnarySuffix(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_unary_prefix => Self::UnaryPrefix(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_field_expr => Self::FieldExpression(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_concat_expression => Self::ConcatExpression(Box::new(Parsed::parse(state, node)?)),
                SyntaxKind::sym_choice_expression => Self::ChoiceExpression(Box::new(Parsed::parse(state, node)?)),

                SyntaxKind::anon_sym_LPAREN => continue,
                _ => {
                    println!("{}", node.to_sexp());
                    unimplemented!("SyntaxKind::{:#?}=>{{}}", SyntaxKind::from(&node))
                }
            };
            return Ok(out);
        }
        unreachable!()
    }
}

parsed_wrap!(ConcatExpression:
    lhs << (named_one, "lhs"),
    op << (named_one, "op"),
    rhs << (named_one, "rhs")
);

parsed_wrap!(ChoiceExpression:
    lhs << (named_one, "lhs"),
    op << (named_one, "op"),
    rhs << (named_one, "rhs")
);

parsed_wrap!(ChoiceTag:
    expr << (named_one, "expression"),
    tag << (named_some, "tag"),
    mode << (named_some, "mode"),
    ty << (named_some, "ty")
);

parsed_wrap!(FieldExpression:
    lhs << (named_one, "lhs"),
    op << (named_one, "op"),
    rhs << (named_one, "rhs")
);

parsed_wrap!(UnarySuffix:
    suffix << (named_one, "suffix"),
    base << (named_one, "base")
);

parsed_wrap!(UnaryPrefix:
    prefix << (named_one, "prefix"),
    base << (named_one, "base")
);

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

parsed_wrap!(Identifier: data << parse);
parsed_wrap!(Unsigned: data << parse);
parsed_wrap!(StringRanged: data << parse);

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

impl Parsed for bool {
    fn parse(state: &mut YGGBuilder, this: Node) -> Result<Self> {
        match this.utf8_text(state.text.as_bytes())? {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => unreachable!(),
        }
    }
}
