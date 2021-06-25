use super::*;
use lsp_types::{Range,Position};

impl<N:CSTNode> PositionSystem<N> for Range {
    fn from(node: N) -> Self {
        let s = node.get_range();
        Self {
            start: Position {
                line: s.0 as u32,
                character: s.1 as u32
            },
            end: Position {
                line: s.2 as u32,
                character: s.3 as u32
            }
        }
    }

    fn join(self, rhs: Self) -> Self {
        Self { start: self.start, end: rhs.end }
    }
}

