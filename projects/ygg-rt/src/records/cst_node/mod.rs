use std::{
    fmt::Debug,
    ops::Range,
};

#[derive(Clone, Debug)]
pub struct CSTArena<K> {
    id: usize,
    text: String,
    nodes: Vec<CSTNode<K>>,
}

impl<K> CSTArena<K> {
    pub fn new(text: String) -> Self {
        Self {
            id: 1,
            text,
            nodes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CSTNode<K> {
    kind: K,
    parent: usize,
    children: Vec<usize>,
    range: Range<usize>,
}

impl<L> CSTNode<L> {
    pub fn child_by_kinds<K>(&self, kinds: &[K]) -> Option<ASTNode<K>> {
        self.children
            .iter()
            .find(|child| kinds.contains(&child.kind))
            .cloned()
    }
    pub fn children_by_kinds(&self, kinds: &[K]) -> Vec<ASTNode<K>> {
        self.children
            .iter()
            .filter(|child| kinds.contains(&child.kind))
            .cloned()
            .collect()
    }
}