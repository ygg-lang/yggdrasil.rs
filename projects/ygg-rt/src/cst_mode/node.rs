use super::*;

/// Note that these APIs are not designed for humans, please use the code generation solution
impl<K> ConcreteNode<K> {
    pub fn new(kind: K) -> Self {
        Self { kind, node_tag: "", branch: "", range: Default::default() }
    }
    pub fn append_to(self, tree: &mut ConcreteTree<K>, parent: NodeId) -> NodeId {
        let this = tree.arena.new_node(self);
        parent.append(this, &mut tree.arena);
        this
    }
}
