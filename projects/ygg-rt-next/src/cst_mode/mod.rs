use crate::TextStream;
use alloc::{rc::Rc, vec::Vec};
use core::ops::Range;
mod iters;
mod nodes;
mod trees;

#[derive(Clone)]
pub struct ConcreteTree<Language, Input> {
    input: Input,
    language: Language,
    arena: Vec<ConcreteNodeData>,
}

#[derive(Clone)]
pub(crate) struct ConcreteNodeData {
    this: usize,
    parent: Option<usize>,
    prev: Option<usize>,
    next: Option<usize>,
    brother: Range<usize>,
    children: Option<Range<usize>>,
    rule: u32,
    span: Range<usize>,
}

pub struct ConcreteNode<Language, Input> {
    shared: Rc<ConcreteTree<Language, Input>>,
    id: usize,
}

impl<Language, Input> ConcreteTree<Language, Input> {
    pub fn get_node(self: Rc<Self>, id: usize) -> Option<ConcreteNode<Language, Input>> {
        let data = self.arena.get(id)?;
        Some(ConcreteNode { shared: self.clone(), id: data.this })
    }
}

pub struct ConcreteNodeIterator<Language, Input> {
    skip_hidden: bool,
    shared: Rc<ConcreteTree<Language, Input>>,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct ConcreteNodeTraverser<Language, Input> {
    dfs: bool,
    shared: Rc<ConcreteTree<Language, Input>>,
    prev: Option<usize>,
    next: Option<usize>,
}
