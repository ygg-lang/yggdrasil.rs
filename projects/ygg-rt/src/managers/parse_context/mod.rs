use crate::{CstNode, NodeID, NodeManager, NodeType};
use pex::ParseState;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::marker::PhantomData;

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
        self.node_stack.pop().unwrap()
    }
    pub fn get_scope(&self) -> Option<NodeID> {
        self.node_stack.last().map(|n| n.id)
    }
    pub fn add_node(&mut self, id: NodeID, kind: N, start: ParseState, end: ParseState) -> NodeID {
        let node = CstNode::new(id).with_kind(kind).with_range(start.start_offset, end.start_offset);
        self.manager.add_node(node)
    }
}
