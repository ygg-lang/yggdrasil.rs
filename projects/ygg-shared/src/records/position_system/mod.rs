#[cfg(feature = "lsp")]
mod lsp_range;
mod offset;

pub use self::offset::OffsetRange;
#[cfg(feature = "lsp")]
pub use lsp_types::Range as LSPRange;

use crate::traits::{CSTNode, PositionSystem};
use std::fmt::{Debug, Formatter};

pub fn get_position<T, N>(node: N) -> T
where
    T: PositionSystem<N>,
    N: CSTNode,
{
    PositionSystem::from(node)
}

pub fn join_position<T, N>(lhs: T, rhs: T) -> T
where
    T: PositionSystem<N>,
{
    lhs.join(rhs)
}
