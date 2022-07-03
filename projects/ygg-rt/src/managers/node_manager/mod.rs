use crate::{CstNode, LanguageID};
use dashmap::DashMap;
use std::fmt::Debug;

use std::ops::Range;

pub type NodeID = u32;

pub trait NodeType: Copy + Debug + Into<i16> + From<i16> {
    fn get_language_id(&self) -> LanguageID;
    fn is_ignored(&self) -> bool;
}

#[derive(Debug)]
pub struct NodeManager {
    arena: DashMap<NodeID, CstNode>,
}

impl Default for NodeManager {
    fn default() -> Self {
        Self { arena: DashMap::new() }
    }
}

impl NodeManager {
    /// Get the node from the arena
    pub fn get_node(&self, id: NodeID) -> Option<CstNode> {
        self.arena.get(&id).map(|x| x.value().clone())
    }
    /// Add new node to the arena
    pub fn add_node(&self, node: CstNode) -> NodeID {
        let id = node.get_id();
        self.arena.insert(id, node);
        id
    }
    pub fn contains(&self, id: NodeID) -> bool {
        self.arena.contains_key(&id)
    }
    /// Check if the node has no children
    pub fn is_leaf(&self, id: NodeID) -> bool {
        self.find_child(id).is_none()
    }
    /// Check if the node is in the arena
    pub fn is_node(&self, id: NodeID) -> bool {
        self.find_child(id).is_some()
    }
    /// Check if the node has no parent
    pub fn is_root(&self, id: NodeID) -> bool {
        self.find_parent(id).is_none()
    }
    /// Find the parent of the node
    pub fn find_parent(&self, id: NodeID) -> Option<NodeID> {
        let mut find = None;
        for node in self.arena.iter() {
            if node.value().children.contains(&id) {
                if cfg!(debug_assertions) {
                    match find {
                        Some(_) => panic!("Node has multiple parents"),
                        None => find = Some(node.key().clone()),
                    }
                }
                return Some(node.key().clone());
            }
        }
        find
    }
    /// Find the ancestors of the node
    pub fn find_ancestors(&self, id: NodeID, depth: usize) -> Vec<NodeID> {
        let mut ancestors = Vec::new();
        let mut current = id;
        for _ in 0..depth {
            if let Some(parent) = self.find_parent(current) {
                ancestors.push(parent);
                current = parent;
            }
        }
        ancestors
    }
    /// Find the first child of the node
    pub fn find_child(&self, id: NodeID) -> Option<NodeID> {
        self.get_node(id)?.get_children().first().copied()
    }
    /// Find all the children of the node
    pub fn find_children(&self, id: NodeID) -> Vec<NodeID> {
        match self.get_node(id) {
            Some(s) => s.get_children().to_vec(),
            None => vec![],
        }
    }
    /// Find the descendants of the node
    pub fn find_descendants(&self, id: NodeID, depth: usize) -> Vec<NodeID> {
        let mut descendants = Vec::new();
        let mut current = vec![id];
        for _ in 0..depth {
            let mut next = Vec::new();
            for node in current {
                if let Some(child) = self.find_child(node) {
                    descendants.push(child);
                    next.push(child);
                }
            }
            current = next;
        }
        descendants
    }
    pub fn filter_child<F>(&self, id: NodeID, filter: F) -> Option<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        for child in self.find_children(id) {
            if let Some(node) = self.get_node(child) {
                if filter(&node) {
                    return Some(node);
                }
            }
        }
        None
    }
    pub fn filter_children<F>(&self, id: NodeID, filter: F) -> Vec<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        let mut out = Vec::new();
        for child in self.find_children(id) {
            if let Some(node) = self.get_node(child) {
                if filter(&node) {
                    out.push(node);
                }
            }
        }
        out
    }
    pub fn get_range(&self, id: NodeID) -> Option<Range<usize>> {
        Some(self.get_node(id)?.get_range())
    }
    pub fn in_range(&self, offset: usize, id: NodeID) -> bool {
        match self.get_node(id) {
            Some(node) => node.get_range().contains(&offset),
            None => false,
        }
    }
    pub fn update_range(&self, node: &mut CstNode) -> bool {
        let out: Option<(u32, u32)> = try {
            let first = self.arena.get(node.children.first()?)?.value().range.start;
            let last = self.arena.get(node.children.last()?)?.value().range.end;
            (first, last)
        };
        match out {
            Some((start, end)) => {
                node.range = start..end;
                true
            }
            None => false,
        }
    }
}
