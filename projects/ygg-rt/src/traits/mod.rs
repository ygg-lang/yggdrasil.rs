mod position;
mod pratt;

pub use self::pratt::{Affix, PrattParser};
use std::{fmt::Debug, ops::Range};

pub trait NodeType: Copy + Debug {
    fn is_ignored(&self) -> bool;
}

pub trait AstNode
where
    Self: Sized,
{
    type NodeType: NodeType;
    const KIND: Self::NodeType;
    fn get_range(&self) -> Range<usize>;
}
