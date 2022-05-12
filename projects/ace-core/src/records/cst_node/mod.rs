use std::{
    fmt::Debug,
    ops::Range,
};

pub trait LanguageType {
    fn language_id() -> &'static str;
}

pub struct RustLanguage {}

impl LanguageType for RustLanguage {
    fn language_id() -> LanguageID {
        LanguageID("rust")
    }
}

pub struct LanguageID(&'static str);

impl From<String> for LanguageID {
    fn from(s: String) -> Self {
        Self(s.as_str())
    }
}

pub struct FileID(usize);

pub struct NodeID {
    file: FileID,
    this: usize,
}

#[derive(Clone, Debug)]
pub struct PsiFile {
    file: FileID,
    nodes: Vec<AceNode>,
}

impl<K> PsiFile<K> {
    pub fn new(text: String) -> Self {
        Self {
            file: FileID(0),
            nodes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AceNode {
    kind: usize,
    this: NodeID,
    parent: Option<NodeID>,
    children: Vec<NodeID>,
    range: Range<usize>,
}

pub trait CSTKind: Into<usize> + PartialEq {
    fn is_ignored(&self) -> bool {
        self.is_comment() || self.is_whitespace()
    }

    fn is_comment(&self) -> bool;

    fn is_whitespace(&self) -> bool;

    fn is_keyword(&self) -> bool {
        false
    }
}

impl<K> AceNode<K> where K: CSTKind {
    pub fn get_parent<'a>(&self, arena: &'a PsiFile<K>) -> Option<&'a AceNode<K>> {
        self.parent.map(|parent| &arena.nodes[parent.get() - 1])
    }
    pub fn set_parent(&mut self, parent: NodeID) {
        self.parent = Some(parent);
    }
    pub fn ancestors<'a>(&self, arena: &'a PsiFile<K>) -> impl Iterator<Item=&'a AceNode<K>> {
        let mut parent = self.get_parent(arena);
        std::iter::from_fn(move || {
            let result = parent;
            parent = parent.and_then(|parent| parent.get_parent(arena));
            result
        })
    }
    pub fn leaf_by_kinds<'a>(&self, arena: &'a PsiFile<K>, kinds: &[K]) -> Option<&'a AceNode<K>> {
        self.children
            .iter()
            .find(|child| kinds.contains(&arena.nodes[**child].kind))
            .map(|child| &arena.nodes[*child])
    }
    pub fn leaves_by_kinds<'a>(&self, arena: &'a PsiFile<K>, kinds: &[K]) -> impl Iterator<Item=&'a AceNode<K>> {
        self.children
            .iter()
            .filter(|child| kinds.contains(&arena.nodes[**child].kind))
            .map(|child| &arena.nodes[*child])
    }
    pub fn children<'a>(&self, arena: &'a PsiFile<K>) -> Vec<&'a AceNode<K>> {
        self.leaves_by_kinds(arena, &[]).collect()
    }
}