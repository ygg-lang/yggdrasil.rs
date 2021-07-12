mod node;

use crate::{records::CSTNode, Result};

pub use self::node::ASTNode;

pub trait PositionSystem<N> {
    /// The middle way to avoid the orphan rule
    fn from(node: N) -> Self;
    fn join(self, rhs: Self) -> Self;
}

pub trait CSTParser<R> {
    fn parse(&mut self, input: &str) -> Result<CSTNode<R>>;
}
