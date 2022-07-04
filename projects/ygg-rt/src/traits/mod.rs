mod position;
mod pratt;

pub use self::pratt::{Affix, PrattParser};
use crate::{CstContext, CstNode, LanguageID, NodeID};
use std::{fmt::Debug, ops::Range};

pub trait NodeType: Copy + Debug + Into<i16> + From<i16> {
    fn get_language_id(&self) -> LanguageID;
    fn is_ignored(&self) -> bool;
}

pub trait AstNode
where
    Self: Sized,
{
    type NodeType: NodeType;
    const KIND: Self::NodeType;
    fn from_cst(ctx: &CstContext<Self::NodeType>, parent: NodeID) -> Self;
    fn get_range(&self) -> Range<usize>;
    fn get_language_id() -> LanguageID {
        Self::KIND.get_language_id()
    }
}
