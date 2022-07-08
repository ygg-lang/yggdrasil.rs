use crate::NodeType;
use pex::ParseState;
use std::{
    num::{NonZeroU16, NonZeroU32},
    ops::Range,
};

pub struct ConcreteTree<K> {
    text: String,
    root: Vec<ConcreteID>,
    tags: &'static [&'static str],
    branches: &'static [&'static str],
    kind: std::marker::PhantomData<K>,
}

impl<K> ConcreteTree<K> {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string(), root: vec![], tags: &[], branches: &[], kind: Default::default() }
    }
    pub fn parse_state(&self) -> ParseState {
        ParseState::new(self.text.as_str())
    }
}

#[repr(C)]
pub struct NodeTag {
    node: u16,
    branch: u16,
}

#[repr(C)]
pub struct ConcreteID {
    id: u32,      // 32
    kind: u32,    // 32
    tag: NodeTag, // 32
    _fill: u32,   // 32 for align, useless
    start: u32,   // 32
    end: u32,     // 32
}

impl ConcreteID {
    #[track_caller]
    pub fn get_node_tag<K>(&self, tree: &ConcreteTree<K>) -> &'static str {
        debug_assert!(self.tag.node < 65535);
        unsafe { tree.tags.get_unchecked(self.tag.node as usize) }
    }
    #[track_caller]
    pub fn get_branch_tag<K>(&self, tree: &ConcreteTree<K>) -> &'static str {
        debug_assert!(self.tag.node < 65535);
        unsafe { tree.branches.get_unchecked(self.tag.branch as usize) }
    }
    #[track_caller]
    pub fn get_text<'i, K>(&self, tree: &'i ConcreteTree<K>) -> &'i str {
        let start = self.start as usize;
        let end = self.end as usize;
        assert!(start <= end && end <= tree.text.len());
        unsafe { tree.text.get_unchecked(start..end) }
    }
}

#[test]
fn keep_size() {
    assert_eq!(std::mem::size_of::<NodeTag>() * 8, 32);
    assert_eq!(std::mem::size_of::<ConcreteID>() * 8, 64 * 3);
}

#[repr(u32)]
pub enum CalculateKind {
    Add = 0,
    Number = 1,
    Space = 100,
}

#[test]
fn test() {
    let mut tree = ConcreteTree::<u32>::new("1 + 1");
    let state = tree.parse_state();
}
