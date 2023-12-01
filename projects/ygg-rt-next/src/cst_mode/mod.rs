use crate::InputStream;
use alloc::{rc::Rc, vec::Vec};
use core::ops::Range;
mod iters;
mod nodes;

pub trait YggdrasilRule: TryFrom<u32> {}

pub trait YggdrasilLanguage {
    type Rule: YggdrasilRule;
}

pub struct ConcreteTree<Language, Input> {
    input: Input,
    language: Language,
    arena: Vec<ConcreteNodeData>,
}

pub struct ConcreteNodeData {
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

pub struct ConcreteNodeIterator<Language, Input> {
    shared: Rc<ConcreteTree<Language, Input>>,
    prev: Option<usize>,
    next: Option<usize>,
}
