
use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OffsetRange {
    pub start: usize,
    pub end: usize,
}



impl<R> PositionSystem<R> for OffsetRange {
    fn from(pair: &Pair<R>) -> Self {
        let s = pair.as_span();
        Self {
            start: s.start(),
            end: s.end(),
        }
    }
}