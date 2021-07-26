use super::*;
use crate::records::CSTNode;

pub trait PositionSystem {
    fn get_span(&self) -> (usize, usize);
    fn get_start(&self) -> usize;
    fn get_end(&self) -> usize;
}

impl<R> PositionSystem for CSTNode<R> {
    /// Provide basic location information
    /// (start_offset, end_offset)
    fn get_span(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    fn get_start(&self) -> usize {
        self.start
    }
    fn get_end(&self) -> usize {
        self.end
    }
}
