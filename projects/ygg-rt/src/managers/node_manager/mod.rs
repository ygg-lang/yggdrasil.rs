use crate::{CSTNode, LanguageID};
use dashmap::DashMap;

pub type NodeID = usize;

pub trait NodeType: Copy + Into<usize> + From<usize> {
    fn get_language_id(&self) -> LanguageID;
    fn is_ignored(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct NodeManager {
    arena: DashMap<NodeID, CSTNode>,
    /// The root node of the cst
    parents: DashMap<NodeID, NodeID>,
}

impl NodeManager {
    /// Get the node from the arena
    pub fn get_node(&self, id: NodeID) -> Option<CSTNode> {
        self.arena.get(&id).map(|x| x.value().clone())
    }
    /// Add new node to the arena
    pub fn add_node(&mut self, node: CSTNode, parent: Option<NodeID>) -> NodeID {
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
    pub fn filter_child<F>(&self, id: NodeID, filter: F) -> Option<CSTNode>
    where
        F: Fn(&CSTNode) -> bool,
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
    pub fn filter_children<F>(&self, id: NodeID, filter: F) -> Vec<CSTNode>
    where
        F: Fn(&CSTNode) -> bool,
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
