use super::*;

impl<N: NodeType> Default for CstTyped<N> {
    fn default() -> Self {
        Self { id: 0, kind: N::from(0), children: vec![], range: 0..0, text: "".to_string() }
    }
}

impl<N: NodeType> Debug for CstTyped<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.id == 0 {
            return f.debug_struct("Missing").finish();
        }
        if self.children.is_empty() {
            return f
                .debug_struct("Leaf")
                .field("id", &self.id)
                .field("kind", &self.kind)
                .field("text", &self.text)
                .field("range", &self.range)
                .finish();
        }
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("children", &self.children)
            .field("range", &self.range)
            .finish()
    }
}
