use super::*;

impl YggdrasilCST {
    pub fn extract_namespace(&self, id: NodeId) -> Option<YggdrasilNamespace> {
        let cst = self.tree.get_node(id)?;
        debug_assert!(cst.kind == YggdrasilType::Namespace, "Expected `YggdrasilType::Namespace`, got `{:?}`", cst.kind);
        let mut identifiers = Vec::new();
        for (id, child) in self.tree.children(id) {
            if child.kind == YggdrasilType::Identifier {
                match self.extract_identifier(id) {
                    Some(s) => identifiers.push(s),
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
}
