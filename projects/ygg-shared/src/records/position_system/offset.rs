use crate::traits::CSTNode;
use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OffsetRange {
    pub start: usize,
    pub end: usize,
}

impl<N:CSTNode> PositionSystem<N> for OffsetRange {
    fn from(node: N) -> Self {
        let s = node.get_span();
        Self { start: s.0, end: s.1 }
    }

    fn join(self, rhs: Self) -> Self {
        Self { start: self.start, end: rhs.end }
    }
}

impl Default for OffsetRange {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}

impl Debug for OffsetRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Offset({}-{})", self.start, self.end)
    }
}
