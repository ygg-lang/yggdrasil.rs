use pest::RuleType;
use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OffsetRange {
    pub start: usize,
    pub end: usize,
}

impl<R> PositionSystem<R> for OffsetRange
where R:RuleType {
    fn from(pair: &Pair<R>) -> Self {
        let s = pair.as_span();
        Self {
            start: s.start(),
            end: s.end(),
        }
    }
}