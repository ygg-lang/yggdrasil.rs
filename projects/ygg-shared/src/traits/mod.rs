mod node;
mod pratt;

pub use self::{
    node::ASTNode,
    pratt::{Affix, Associativity, PrattParser, Precedence},
};

pub trait PositionSystem<N> {
    /// The middle way to avoid the orphan rule
    fn from(node: N) -> Self;
    fn join(self, rhs: Self) -> Self;
}
