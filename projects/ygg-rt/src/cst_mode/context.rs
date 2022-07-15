use super::*;

impl<K> ConcreteTree<K>
where
    K: NodeType,
{
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string(), root: None, arena: RefCell::new(Arena::new()) }
    }
    pub fn initial_state(&self) -> ParseState {
        ParseState::new(self.text.as_str())
    }
    pub fn create_root(&mut self, kind: K) -> NodeId {
        match self.root {
            Some(_) => panic!("ConcreteTree: Root node already exists"),
            None => {
                let mut empty = ConcreteNode::new(kind);
                empty.range = 0..self.text.len();
                let root_id = self.create_node(empty);
                self.root = Some(root_id);
                root_id
            }
        }
    }
    pub fn get_root(&self) -> Option<(NodeId, ConcreteNode<K>)> {
        let id = self.root?;
        let node = self.get_node(id)?;
        Some((id, node))
    }
    pub fn create_node(&self, data: ConcreteNode<K>) -> NodeId {
        self.arena.borrow_mut().new_node(data)
    }
    pub fn get_node(&self, node: NodeId) -> Option<ConcreteNode<K>> {
        Some(self.arena.borrow().get(node)?.get().clone())
    }
    /// An allocation is always required when converting an ast, so no iterators are used here
    pub fn children(&self, node: NodeId) -> Vec<(NodeId, ConcreteNode<K>)> {
        // for filling binary like expressions (a + b) => (a ~ + ~ b ~)
        let mut out = Vec::with_capacity(6);
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
    pub fn place_holder_node(&self, parent: NodeId) -> NodeId {
        let empty = self.empty_node();
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

impl<K> ConcreteTree<K>
where
    K: NodeType,
{
    fn empty_node(&self) -> NodeId {
        self.arena.borrow_mut().new_node(ConcreteNode {
            kind: K::default(),
            node_tag: "",
            branch_tag: "",
            range: Default::default(),
        })
    }
}
