use super::*;

impl<K> ConcreteTree<K> {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string(), arena: Arena::new() }
    }
    pub fn parse_state(&self) -> ParseState {
        ParseState::new(self.text.as_str())
    }
    pub fn create_root_node(&mut self, data: ConcreteNode<K>) -> NodeId
    where
        K: NodeType,
    {
        self.arena.new_node(data)
    }
    pub fn place_holder_node(&mut self, parent: NodeId, kind: K) -> NodeId {
        let empty = self.arena.new_node(ConcreteNode { kind, node_tag: "", branch: "", range: Default::default() });
        parent.append(empty, &mut self.arena);
        empty
    }
    pub fn update_node(&mut self, old: NodeId, data: ConcreteNode<K>) {
        match self.arena.get_mut(old) {
            Some(s) => {
                *s.get_mut() = data;
            }
            None => {
                if cfg!(debug_assertions) {
                    panic!("update_node: node {old} not found");
                }
            }
        }
    }
    pub fn append_node(&mut self, parent: NodeId, child: NodeId) {
        parent.append(child, &mut self.arena);
    }
    pub fn drop_node(&mut self, node: NodeId) {
        node.remove_subtree(&mut self.arena);
    }
}
