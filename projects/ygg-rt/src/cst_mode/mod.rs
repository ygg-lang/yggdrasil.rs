use std::ops::Range;

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
    id: usize,
    /// An enum that implements the [`NodeType`]
    kind: usize,
    /// The offset in raw bytes, life time erased
    start: usize,
    end: usize,
}



impl CSTNode {
    /// Create a new cst node
    pub fn new(id: usize) -> Self {
        Self {
            id,
            kind: 0,
            start: 0,
            end: 0,
        }
    }
    pub fn with_kind<N>(mut self, kind: N) -> Self
    where
        N: NodeType,
    {
        self.kind = kind.into();
        self
    }
    pub fn get_range(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        // range not copy
        self.start = start;
        self.end = end;
        self
    }
}

impl CSTNode {
    pub fn view<'i>(&self, input: &'i str) -> &'i str {
        &input[self.range.clone()]
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn collect_ids(&self) -> Vec<usize> {
        let mut ids = vec![self.id];
        for node in self.nodes.iter() {
            ids.extend(node.collect_ids());
        }
        ids
    }
    pub fn find<N>(&self, types: &[N]) -> Option<&CSTNode>
    where
        N: NodeType,
    {
        for node in &self.nodes {
            if self.is_a(types) {
                return Some(node);
            }
        }
        return None;
    }
    pub fn filter<'s, 'i, N>(&'s self, types: &'i [N]) -> impl Iterator<Item = &CSTNode> + 'i
    where
        N: NodeType,
        's: 'i,
    {
        self.nodes.iter().filter(|child| child.is_a(types))
    }

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
