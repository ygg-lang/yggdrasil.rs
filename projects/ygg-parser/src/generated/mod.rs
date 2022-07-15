mod ast;
mod cst;
mod extensions;
mod trait_bounds;

pub use self::extensions::*;
use std::{hash::Hash, ops::Range};
use yggdrasil_rt::{AstNode, ConcreteNode, ConcreteTree, NodeId, NodeType, ParseResult, ParseState};

#[repr(u16)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Identifier = 10,
    Namespace = 100,
    WhiteSpace = 1000,
    Literal = 11111,
    Missing = 666,
}

pub struct YggdrasilCST {
    tree: ConcreteTree<YggdrasilType>,
}

#[derive(Debug)]
pub struct YggdrasilNamespace {
    #[cfg(debug_assertions)]
    pub text: String,
    pub identifiers: Vec<YggdrasilIdentifier>,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct YggdrasilIdentifier {
    #[cfg(debug_assertions)]
    pub text: String,
    pub range: Range<usize>,
}

#[test]
fn test() {
    let text = "c::i + j";
    let state = ParseState::new(text);
    let mut ctx = YggdrasilCST { tree: ConcreteTree::<YggdrasilType>::new(text) };
    let root = ctx.tree.create_root(YggdrasilType::Program);
    let result = ctx.parse_namepath(state, root).as_result().unwrap();
    println!("{}", ctx.tree);
    println!("{:#?}", result);
    let ns = ctx.extract_namespace(result.1);
    println!("{:#?}", ns);
}
