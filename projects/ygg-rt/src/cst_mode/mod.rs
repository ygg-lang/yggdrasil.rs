use crate::NodeType;
use std::{fmt::Debug, ops::Range};

use indextree::{Arena, Node, NodeId};
use pex::{ParseResult, ParseState};
use std::{
    mem::transmute,
    num::{NonZeroU16, NonZeroU32},
};

pub mod context;
mod display;
pub mod node;

pub struct ConcreteTree<K> {
    text: String,
    arena: Arena<ConcreteNode<K>>,
}

/// A compact node representation
#[derive(Clone, Debug)]
pub struct ConcreteNode<K> {
    kind: K,
    node_tag: &'static str,
    branch: &'static str,
    range: Range<usize>,
}
