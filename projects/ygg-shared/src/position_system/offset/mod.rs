use super::*;
use pest::RuleType;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OffsetRange {
    pub start: usize,
    pub end: usize,
}

impl<R> PositionSystem<R> for OffsetRange
where
    R: RuleType,
{
    fn from(pair: &Pair<R>) -> Self {
        let s = pair.as_span();
        Self { start: s.start(), end: s.end() }
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
