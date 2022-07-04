use crate::{cst_mode::CstTyped, AstNode, CstNode, NodeID, NodeManager, NodeType};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::Range,
};

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
        self.manager.get_node(&id)
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
    pub fn filter_child<F>(&self, node: NodeID, filter: F) -> Option<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        let s = self.get_node(node)?;
        s.children.iter().filter_map(|c| self.get_node(*c)).find(filter)
    }
    pub fn filter_children<F>(&self, node: NodeID, filter: F) -> Vec<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        match self.get_node(node) {
            Some(s) => s.children.iter().filter_map(|c| self.get_node(*c)).filter(filter).collect(),
            None => {
                vec![]
            }
        }
    }
    pub fn get_child<A>(&self, node: NodeID) -> Option<A>
    where
        A: AstNode<NodeType = N>,
    {
        let n = self.filter_child(node, |n| n.is_a(&[A::KIND]))?;
        Some(A::from_cst(self, n.id))
    }

    pub fn get_children<A>(&self, node: NodeID) -> Vec<A>
    where
        A: AstNode<NodeType = N>,
    {
        self.filter_children(node, |n| n.is_a(&[A::KIND])).into_iter().map(|c| A::from_cst(self, c.id)).collect()
    }
}
