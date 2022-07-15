use super::*;

impl Debug for YggdrasilCST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.tree, f)
    }
}

impl Display for YggdrasilCST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.tree, f)
    }
}

impl Default for YggdrasilType {
    fn default() -> Self {
        YggdrasilType::Uninitialized
    }
}

impl NodeType for YggdrasilType {
    fn is_ignored(&self) -> bool {
        match self {
            YggdrasilType::Program => false,
            YggdrasilType::Identifier => false,
            YggdrasilType::Namespace => false,
            YggdrasilType::WhiteSpace => true,
            YggdrasilType::Literal => true,
            YggdrasilType::Uninitialized => true,
            YggdrasilType::Number => false,
        }
    }
}

impl AstNode for YggdrasilNamespace {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Namespace;

    fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
}
