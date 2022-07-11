use super::*;

/// Note that these APIs are not designed for humans, please use the code generation solution
impl<K> ConcreteNode<K> {
    pub fn new(kind: K) -> Self {
        Self { kind, node_tag: "", branch_tag: "", range: Default::default() }
    }
    pub fn with_offset(mut self, old: ParseState, new: ParseState) -> Self {
        self.range = new.away_from(old);
        self
    }
}
