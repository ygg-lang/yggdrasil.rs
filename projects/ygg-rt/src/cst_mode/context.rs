use super::*;

impl<K> ConcreteTree<K> {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string(), arena: RefCell::new(Arena::new()) }
    }
    pub fn parse_state(&self) -> ParseState {
        ParseState::new(self.text.as_str())
    }
    pub fn create_node(&self, data: ConcreteNode<K>) -> NodeId
    where
        K: NodeType,
    {
        self.arena.borrow_mut().new_node(data)
    }
    pub fn get_root(&self) -> NodeId {
        self.arena.borrow().root_id()
    }
    pub fn get_node(&self, node: NodeId) -> Option<ConcreteNode<K>>
    where
        K: NodeType,
    {
        Some(self.arena.borrow().get(node)?.get().clone())
    }
    /// An allocation is always required when converting an ast, so no iterators are used here
    pub fn children(&self, node: NodeId) -> Vec<(NodeId, ConcreteNode<K>)>
    where
        K: NodeType,
    {
        /// should at least greater than 2, for filling binary expressions
        let mut out = Vec::with_capacity(4);
        for child in node.children(&self.arena.borrow()) {
            match self.get_node(child) {
                Some(s) => out.push((child, s)),
                None => {
                    if cfg!(debug_assertions) {
                        panic!("get::children: node {child} not found");
                    }
                }
            }
        }
        out
    }
    pub fn get_text(&self, range: &Range<usize>) -> &str {
        match self.text.get(range.clone()) {
            Some(s) => s,
            None => "",
        }
    }
    pub fn get_snippet(&self, range: &Range<usize>) -> String {
        const MAX_LENGTH: usize = 63;
        const END_LENGTH: usize = 10; // if larger than MAX_LENGTH, fill with ...+END_LENGTH
        const MIDDLE_FILL: &str = "...";
        let text = self.get_text(range);
        if text.len() <= MAX_LENGTH {
            return text.to_string();
        }
        let mut out = String::with_capacity(MAX_LENGTH);
        out.push_str(&text[..MAX_LENGTH - END_LENGTH]);
        out.push_str(MIDDLE_FILL);
        out.push_str(&text[text.len() - END_LENGTH..]);
        out
    }

    pub fn place_holder_node(&self, parent: NodeId) -> NodeId
    where
        K: Default,
    {
        let empty = self.arena.borrow_mut().new_node(ConcreteNode {
            kind: K::default(),
            node_tag: "",
            branch: "",
            range: Default::default(),
        });
        parent.append(empty, &mut self.arena.borrow_mut());
        empty
    }
    pub fn update_node(&self, old: NodeId, data: ConcreteNode<K>) {
        match self.arena.borrow_mut().get_mut(old) {
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
    pub fn append_node(&self, parent: NodeId, child: NodeId) {
        parent.append(child, &mut self.arena.borrow_mut());
    }
    pub fn drop_node(&self, node: NodeId) {
        node.remove_subtree(&mut self.arena.borrow_mut());
    }
}
