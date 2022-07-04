use crate::{AstNode, CstNode, NodeID, NodeManager, NodeType};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{marker::PhantomData, ops::Range};

pub struct CstContext<'a, N: NodeType> {
    manager: &'a NodeManager,
    random: SmallRng,
    node_stack: Vec<CstNode>,
    node_type: PhantomData<N>,
}

impl<'a, N: NodeType> CstContext<'a, N> {
    pub fn new(manager: &'a NodeManager) -> Self {
        Self { manager, random: SmallRng::from_entropy(), node_stack: vec![], node_type: Default::default() }
    }
    pub fn random_id(&mut self) -> NodeID {
        self.random.gen()
    }
    pub fn random_scope(&mut self) -> NodeID {
        let id = self.random_id();
        self.node_stack.push(CstNode::new(id));
        id
    }
    pub fn end_scope(&mut self) -> CstNode {
        if cfg!(debug_assertions) {
            assert!(!self.node_stack.is_empty());
        }
        unsafe { self.node_stack.pop().unwrap_unchecked() }
    }
    pub fn get_scope(&self) -> Option<NodeID> {
        self.node_stack.last().map(|n| n.id)
    }
    pub fn add_option(&mut self, node: Option<CstNode>) {
        match node {
            Some(node) => {
                self.add_node(node);
            }
            None => {}
        }
    }
    pub fn get_node(&self, id: NodeID) -> Option<CstNode> {
        self.manager.get_node(id)
    }
    pub fn add_node(&mut self, node: CstNode) -> NodeID {
        if cfg!(debug_assertions) {
            assert!(!self.node_stack.is_empty());
        }
        let id = self.manager.add_node(node);
        unsafe {
            self.node_stack.last_mut().unwrap_unchecked().add_child(id);
        }
        id
    }
    pub fn add_root(&mut self, node: CstNode) -> NodeID {
        self.manager.add_root(node)
    }
    pub fn find_range(&self, node: NodeID) -> Range<usize> {
        self.manager.get_range(node)
    }
    pub fn find_child<F>(&self, id: NodeID, filter: F) -> Option<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        self.manager.filter_child(id, filter)
    }
    pub fn find_children<A>(&self, parent: NodeID) -> Vec<A>
    where
        A: AstNode<NodeType = N>,
    {
        match self.get_node(parent) {
            Some(s) => {
                let mut out = Vec::with_capacity(s.children.len());
                for child in s.children.iter() {
                    out.push(A::from_cst(self, *child))
                }
                out
            }
            None => {
                vec![]
            }
        }
    }
}
