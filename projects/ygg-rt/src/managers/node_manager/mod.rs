

pub struct NodeManager {
    /// The root node of the cst
    parents: Dashmap<usize, usize>,
    /// The raw input
    input: String,
}