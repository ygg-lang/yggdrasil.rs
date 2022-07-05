use super::*;

impl<N: NodeType> Default for CstContext<N> {
    fn default() -> Self {
        Self { random: SmallRng::from_entropy(), node_stack: vec![], node_type: PhantomData }
    }
}

impl<N: NodeType> Deref for CstContext<N> {
    type Target = NodeManager;
    fn deref(&self) -> &Self::Target {
        NODE_MANAGER.deref()
    }
}

impl<N: NodeType> CstContext<N> {
    pub fn random_id(&mut self) -> NodeID {
        self.random.gen()
    }
    pub fn random_scope(&mut self) -> NodeID {
        let id = self.random_id();
        self.node_stack.push(CstNode::new(id));
        id
    }
    pub fn end_scope(&mut self) -> CstNode {
        if cfg!(debug_assertions) {
            assert!(!self.node_stack.is_empty());
        }
        unsafe { self.node_stack.pop().unwrap_unchecked() }
    }
    pub fn get_scope(&self) -> Option<NodeID> {
        self.node_stack.last().map(|n| n.id)
    }
    pub fn add_option(&mut self, node: Option<CstNode>) {
        match node {
            Some(node) => {
                self.add_node(node);
            }
            None => {}
        }
    }
    pub fn get_typed(&self, id: NodeID) -> CstTyped<N> {
        NODE_MANAGER.get_typed(&id)
    }
    pub fn add_node(&mut self, node: CstNode) -> NodeID {
        if cfg!(debug_assertions) {
            assert!(!self.node_stack.is_empty());
        }
        let id = NODE_MANAGER.add_node(node);
        unsafe {
            self.node_stack.last_mut().unwrap_unchecked().add_child(id);
        }
        id
    }
    pub fn filter_child<F>(&self, node: NodeID, filter: F) -> Option<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        let s = self.get_node(&node)?;
        s.children.iter().filter_map(|c| self.get_node(c)).find(filter)
    }
    pub fn filter_children<F>(&self, node: NodeID, filter: F) -> Vec<CstNode>
    where
        F: Fn(&CstNode) -> bool,
    {
        match self.get_node(&node) {
            Some(s) => s.children.iter().filter_map(|c| self.get_node(c)).filter(filter).collect(),
            None => {
                vec![]
            }
        }
    }
    pub fn get_child<A>(&self, node: NodeID) -> Option<A>
    where
        A: AstNode<NodeType = N>,
    {
        let n = self.filter_child(node, |n| n.is_a(&[A::KIND]))?;
        Some(A::from_cst(self, n.id))
    }

    pub fn get_children<A>(&self, node: NodeID) -> Vec<A>
    where
        A: AstNode<NodeType = N>,
    {
        self.filter_children(node, |n| n.is_a(&[A::KIND])).into_iter().map(|c| A::from_cst(self, c.id)).collect()
    }
}
