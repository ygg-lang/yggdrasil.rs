use crate::{NodeID, NodeType};
use std::ops::Range;

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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    /// The kind of the node
    kind: usize,
    /// The offset in raw bytes, life time erased
    start: usize,
    /// The offset in raw bytes, life time erased
    end: usize,
}

impl CSTNode {
    /// Create a new cst node
    pub fn new(id: NodeID) -> Self {
        Self { id, kind: 0, start: 0, end: 0 }
    }
    /// Get the id of the node
    pub fn get_id(&self) -> NodeID {
        self.id
    }
    /// Get the kind of the node
    pub fn get_kind<N>(&self) -> N
    where
        N: NodeType,
    {
        <N as From<usize>>::from(self.kind)
    }
    /// Set the kind of the node
    pub fn set_kind<N>(&mut self, kind: N)
    where
        N: NodeType,
    {
        self.kind = <N as Into<usize>>::into(kind);
    }
    /// Set the kind of the node
    pub fn with_kind<N>(mut self, kind: N) -> Self
    where
        N: NodeType,
    {
        self.set_kind(kind);
        self
    }
    /// Get the range of the node
    pub fn get_range(&self) -> Range<usize> {
        self.start..self.end
    }
    /// Set the range of the node
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.start = start;
        self.end = end;
    }
    /// Set the range of the node
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
