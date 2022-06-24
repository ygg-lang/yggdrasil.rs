use crate::NodeID;
use std::{num::NonZeroUsize, ops::Range, rc::Weak, sync::Arc};

pub trait NodeType: Into<usize> + Copy {}

/// The basic unit of semantic analysis.
///
/// A weakly typed node, which can be equivalent to:
///
/// ```
/// pub struct CSTNode<'i, R, M> {
///     meta: M,
///     kind: R,
///     slice: &'i str,
///     children: Vec<CSTNode<'i, R, M>>,
/// }
/// ```
/// Erase theses type to make it more flexible.
///
/// **This node is immutable**
///
/// If a modification occurs, a new clone must be generated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CSTNode {
    /// meta information provided by environment
    /// ```
    /// # use std::collections::BTreeMap;
    /// type LanguageID = usize;
    /// struct LanguageManager {
    ///     cache: BTreeMap<usize, LanguageID>,
    /// }
    /// ```
    /// An enum that implements the [`NodeType`]
    id: usize,
    kind: usize,
    parent: Option<NonZeroUsize>,
    /// The offset in raw bytes, life time erased
    start: usize,
    /// The offset in raw bytes, life time erased
    end: usize,
}

impl CSTNode {
    /// Create a new cst node
    pub fn new<N>(id: NodeID, kind: N) -> Self
    where
        N: NodeType,
    {
        Self { id, kind: <N as Into<usize>>::into(kind), parent: None, start: 0, end: 0 }
    }
    pub fn get_id(&self) -> NodeID {
        self.id
    }
    pub fn get_parent(&self) -> Option<NodeID> {
        self.parent.map(|p| p.get())
    }
    pub fn set_parent(&mut self, parent: Option<NodeID>) {
        self.parent = parent.map(|p| NonZeroUsize::new(p).unwrap());
    }
    /// Get the id of the node
    pub fn with_parent(mut self, parent: Option<NodeID>) -> Self {
        self.set_parent(parent);
        self
    }
    pub fn get_range(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.start = start;
        self.end = end;
    }
    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.set_range(start, end);
        self
    }
}

impl CSTNode {
    /// Check if the node is one of the given types
    ///
    /// # Arguments
    ///
    /// * `kind`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::CSTNode;
    /// enum JsonNode {
    ///     Object,
    ///     Array,
    /// }
    /// let node = CSTNode::new(0).with_kind(JsonNode::Object);
    /// assert!(node.is_a(&[JsonNode::Object]));
    /// ```
    pub fn is_a<N>(&self, kind: &[N]) -> bool
    where
        N: NodeType,
    {
        for node in kind {
            if self.kind == <N as Into<usize>>::into(*node) {
                return true;
            }
        }
        return false;
    }
}
