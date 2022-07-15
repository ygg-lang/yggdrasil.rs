use super::*;

impl YggdrasilCST {
    pub fn extract_value(&self, id: NodeId) -> Option<YggdrasilValue> {
        let cst = self.tree.get_node(id)?;
        let ast = match cst.kind {
            YggdrasilType::Namespace => YggdrasilValue::Namespace(self.extract_namespace(id)?),
            YggdrasilType::Number => YggdrasilValue::Number(self.extract_number(id)?),
            // unreachable
            _ => return None,
        };
        Some(ast)
    }
    pub fn extract_namespace(&self, id: NodeId) -> Option<YggdrasilNamespace> {
        let cst = self.tree.get_node(id)?;
        debug_assert!(cst.kind == YggdrasilType::Namespace, "Expected `YggdrasilType::Namespace`, got `{:?}`", cst.kind);
        let mut identifiers = Vec::new();
        for (id, child) in self.tree.children(id) {
            if child.kind == YggdrasilType::Identifier {
                match self.extract_identifier(id) {
                    Some(s) => identifiers.push(s),
                    // unreachable
                    None => {}
                }
            }
        }
        let ast = YggdrasilNamespace {
            #[cfg(debug_assertions)]
            text: self.tree.get_snippet(&cst.range),
            identifiers,
            range: cst.range.clone(),
        };
        Some(ast)
    }
    pub fn extract_identifier(&self, id: NodeId) -> Option<YggdrasilIdentifier> {
        let cst = self.tree.get_node(id)?;
        debug_assert!(cst.kind == YggdrasilType::Identifier, "Expected `YggdrasilType::Identifier`, got `{:?}`", cst.kind);
        let ast = YggdrasilIdentifier {
            #[cfg(debug_assertions)]
            text: self.tree.get_snippet(&cst.range),
            range: cst.range.clone(),
        };
        Some(ast)
    }
    pub fn extract_number(&self, id: NodeId) -> Option<YggdrasilNumber> {
        let cst = self.tree.get_node(id)?;
        debug_assert!(cst.kind == YggdrasilType::Number, "Expected `YggdrasilType::Identifier`, got `{:?}`", cst.kind);
        let ast = YggdrasilNumber {
            #[cfg(debug_assertions)]
            text: self.tree.get_snippet(&cst.range),
            range: cst.range.clone(),
        };
        Some(ast)
    }
}
