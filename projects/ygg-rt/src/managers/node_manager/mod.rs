use crate::{CstNode, LanguageID};
use dashmap::DashMap;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::sync::Mutex;

pub type NodeID = u32;

pub trait NodeType: Copy + Into<i16> + From<i16> {
    fn get_language_id(&self) -> LanguageID;
    fn is_ignored(&self) -> bool;
}

#[derive(Debug)]
pub struct NodeManager {
    random: Mutex<SmallRng>,
    arena: DashMap<NodeID, CstNode>,
    /// The root node of the cst
    parents: DashMap<NodeID, NodeID>,
}

impl Default for NodeManager {
    fn default() -> Self {
        Self { random: Mutex::new(SmallRng::from_entropy()), arena: DashMap::new(), parents: DashMap::new() }
    }
}

impl NodeManager {
    pub fn random_id(&self) -> NodeID {
        match self.random.try_lock() {
            Ok(mut o) => o.gen(),
            Err(_) => NodeID::default(),
        }
    }
    /// Get the node from the arena
    pub fn get_node(&self, id: NodeID) -> Option<CstNode> {
        self.arena.get(&id).map(|x| x.value().clone())
    }
    /// Add new node to the arena
    pub fn add_node(&self, node: CstNode, parent: Option<NodeID>) -> NodeID {
        let id = node.get_id();
        self.arena.insert(id, node);
        if let Some(parent) = parent {
            self.parents.insert(id, parent);
        }
        id
    }
    /// Check if the node has no children
    pub fn is_leaf(&self, id: NodeID) -> bool {
        self.parents.iter().find(|x| x.key() == &id).is_none()
    }
    /// Check if the node is in the arena
    pub fn is_node(&self, id: NodeID) -> bool {
        !self.is_leaf(id)
    }
    /// Check if the node has no parent
    pub fn is_root(&self, id: NodeID) -> bool {
        self.parents.iter().find(|x| x.value() == &id).is_none()
    }
    /// Find the parent of the node
    pub fn find_parent(&self, id: NodeID) -> Option<NodeID> {
        self.parents.get(&id).map(|x| *x.value())
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
        self.parents.iter().find(|x| x.value() == &id).map(|x| *x.key())
    }
    /// Find all the children of the node
    pub fn find_children(&self, id: NodeID) -> Vec<NodeID> {
        self.parents.iter().filter(|x| x.value() == &id).map(|x| *x.key()).collect()
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
}
