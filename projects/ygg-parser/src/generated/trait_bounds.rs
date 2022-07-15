use super::*;

impl Default for YggdrasilType {
    fn default() -> Self {
        YggdrasilType::Missing
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
            YggdrasilType::Missing => true,
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
