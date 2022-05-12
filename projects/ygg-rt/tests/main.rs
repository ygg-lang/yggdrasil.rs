use std::ops::Deref;
use std::rc::Rc;

use yggdrasil_rt::{CSTArena, CSTNode};

#[test]
fn ready() {
    println!("it, works!")
}

#[repr(u8)]
pub enum ValkyrieKind {
    Program,
    Statement,
    Expression,
    Literal,
    Identifier,
    Operator,
    Keyword,
    Punctuation,
    Whitespace,
    Comment,
    Unknown,
}

struct StatementNode {
    pub(crate) raw: Rc<CSTArena<ValkyrieKind>>,
    pub(crate) proxy: CSTNode<ValkyrieKind>,
}

impl StatementNode {
    pub fn children(&self) -> &[CSTNode<ValkyrieKind>] {
        &self.proxy.children
    }
    pub fn leaves(&self) -> &[CSTNode<ValkyrieKind>] {
        &self.proxy.children
    }
}
