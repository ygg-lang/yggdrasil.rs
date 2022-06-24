use dashmap::DashMap;
use crate::CSTNode;


pub type NodeID = usize;

#[derive(Clone, Debug)]
pub struct NodeManager {
    arena: Vec<CSTNode>,
    /// The root node of the cst
    parents: DashMap<NodeID, NodeID>,
}
