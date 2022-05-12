use std::ops::Deref;
use std::rc::Rc;

use yggdrasil_rt::{PsiFile, AceNode};

#[test]
fn ready() {
    println!("it, works!")
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum ValkyrieKind {
    Program,
    Statement,
    Expression,
    Literal,
    Identifier,
    Operator,
    Keyword,
    Punctuation,
    Whitespace,
    Comment,
    Unknown,
}

struct StatementNode {
    pub(crate) arena: Rc<PsiFile<ValkyrieKind>>,
    pub(crate) proxy: AceNode<ValkyrieKind>,
}

struct KeywordNode {
    pub(crate) arena: Rc<PsiFile<ValkyrieKind>>,
    pub(crate) proxy: AceNode<ValkyrieKind>,
}

impl StatementNode {
    pub fn valid_children(&self) -> &'static [ValkyrieKind] {
        &[ValkyrieKind::Expression, ValkyrieKind::Keyword]
    }
    pub fn get_keyword(&self) -> Vec<KeywordNode> {
        self.proxy.leaves_by_kinds(&self.arena, &[ValkyrieKind::Keyword])
            .into_iter()
            .map(|node| KeywordNode {
                arena: self.arena.clone(),
                proxy: node,
            })
            .collect()
    }
    pub fn get_leaf(&self, nth: usize) -> Option<AceNode<usize>> {
        self.proxy.leaves_by_kinds(&self.arena, &[ValkyrieKind::Keyword])
            .into_iter()
            .map(|node| KeywordNode {
                arena: self.arena.clone(),
                proxy: node,
            })
            .collect()
    }

    pub fn childrens(&self) -> &[AceNode<ValkyrieKind>] {
        self.proxy.leaves_by_kinds(&self.arena, &[ValkyrieKind::Keyword])
            .into_iter()
            .map(|node| KeywordNode {
                arena: self.arena.clone(),
                proxy: node,
            })
            .collect()
    }

    pub fn leaves(&self) -> &[AceNode<ValkyrieKind>] {

    }
}
