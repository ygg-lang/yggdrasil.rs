use std::ops::Range;

pub struct CSTNode {
    kind: usize,
    nodes: Vec<CSTNode>,
    range: Range<usize>,
}
