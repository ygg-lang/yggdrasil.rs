mod node;
mod pratt;

pub use self::node::ASTNode;
pub use self::pratt::{PrattParser, Affix,Associativity,Precedence};

pub trait PositionSystem<N> {
    /// The middle way to avoid the orphan rule
    fn from(node: N) -> Self;
    fn join(self, rhs: Self) -> Self;
}
