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
            YggdrasilType::WhiteSpace => true,
            YggdrasilType::Literal => true,
            YggdrasilType::Uninitialized => true,
            _ => false,
        }
    }
}

impl Debug for YggdrasilValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YggdrasilValue::Namespace(v) => Debug::fmt(v, f),
            YggdrasilValue::Number(v) => Debug::fmt(v, f),
        }
    }
}

impl AstNode for YggdrasilValue {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Value;

    fn get_range(&self) -> Range<usize> {
        match self {
            YggdrasilValue::Namespace(v) => v.get_range(),
            YggdrasilValue::Number(v) => v.get_range(),
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

impl AstNode for YggdrasilNumber {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Number;

    fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
}
