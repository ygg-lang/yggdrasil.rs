use super::*;
use std::fmt::Display;

impl<K> Debug for ConcreteTree<K>
where
    K: NodeType,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConcreteTree").field("text", &self.text).field("arena", &self.arena).finish()
    }
}

impl<K> Display for ConcreteNode<K>
where
    K: NodeType,
{
    /// `<KIND>(RANGE, NODE?, BRANCH>)`
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}..{}", self.node_tag, self.range.start, self.range.end)?;
        if self.node_tag != "" {
            write!(f, ", node: {}", self.branch_tag)?;
        }
        if self.branch_tag != "" {
            write!(f, ", branch: {}", self.branch_tag)?;
        }
        write!(f, ")")
    }
}

impl<K> Display for ConcreteTree<K>
where
    K: NodeType,
{
    //  expr(1..2) text text
    //  ├─ expr
    //  │  ├─ val "u"
    //  │  ├─ op "+"
    //  │  └─ expr
    //  │     ├─ val "v"
    //  │     ├─ op "+"
    //  │     └─ val "w"
    //  ├─ op "+"
    //  ├─ expr
    //  │  ├─ val "x"
    //  │  ├─ op "+"
    //  │  └─ val "y"
    //  ├─ op "+"
    //  └─ val "z"
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let root = self.get_root();
        writeln!(f, "{}", root)?;

        Ok(())
    }
}
