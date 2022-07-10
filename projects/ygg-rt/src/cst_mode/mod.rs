use crate::NodeType;
use std::{fmt::Debug, ops::Range};

use indextree::{Arena, Node, NodeId};
use pex::{ParseResult, ParseState};
use std::{
    cell::RefCell,
    mem::transmute,
    num::{NonZeroU16, NonZeroU32},
};

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
    pub branch: &'static str,
    pub range: Range<usize>,
}
