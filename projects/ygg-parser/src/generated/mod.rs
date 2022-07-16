mod ast;
mod cst;
mod extensions;
mod trait_bounds;

use self::extensions::*;
use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    ops::Range,
};
use yggdrasil_rt::{AstNode, ConcreteNode, ConcreteTree, NodeId, NodeType, ParseResult, ParseState};

#[repr(i16)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 2,
    Identifier = 10,
    Namespace = 100,
    Number = 777,
    Value = 1000,
    Literal = 11111,
    Uninitialized = 0,
    WhiteSpace = -1,
}

pub struct YggdrasilCST {
    tree: ConcreteTree<YggdrasilType>,
}

pub enum YggdrasilValue {
    Namespace(YggdrasilNamespace),
    Number(YggdrasilNumber),
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

#[derive(Debug)]
pub struct YggdrasilNumber {
    #[cfg(debug_assertions)]
    pub text: String,
    pub range: Range<usize>,
}

#[test]
fn test() {
    let text = "c::i + 1";
    let state = ParseState::new(text);
    let mut ctx = YggdrasilCST { tree: ConcreteTree::<YggdrasilType>::new(text) };
    let root = ctx.tree.create_root(YggdrasilType::Program);
    let result = ctx.parse_value(state, root).as_result().unwrap();
    println!("{}", ctx.tree);
    println!("{:#?}", result);
    let ns = ctx.extract_value(result.1);
    println!("{:#?}", ns);
}
