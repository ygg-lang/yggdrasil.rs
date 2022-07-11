use crate::NodeType;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

use indextree::{Arena, NodeId};
use pex::ParseState;
use std::cell::RefCell;

pub mod context;
mod display;
pub mod node;

pub struct ConcreteTree<K> {
    text: String,
    /// HACK: use [RefCell] to skip mutable borrow check
    arena: RefCell<Arena<ConcreteNode<K>>>,
}

/// A compact node representation
#[derive(Clone, Debug)]
pub struct ConcreteNode<K> {
    pub kind: K,
    pub node_tag: &'static str,
    pub branch_tag: &'static str,
    pub range: Range<usize>,
}
