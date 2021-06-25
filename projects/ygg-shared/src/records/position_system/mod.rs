mod offset;
#[cfg(feature = "lsp")]
mod lsp_range;

pub use self::offset::OffsetRange;
#[cfg(feature = "lsp")]
pub use lsp_types::Range as LSPRange;

use std::fmt::{Debug, Formatter};
use crate::traits::{CSTNode, PositionSystem};

pub fn get_position<T, N>(node: N) -> T
where
    T: PositionSystem<N>,
    N: CSTNode
{
    PositionSystem::from(node)
}

pub fn join_position<T, N>(lhs: T, rhs: T) -> T
where
    T: PositionSystem<N>,
{
    lhs.join(rhs)
}


