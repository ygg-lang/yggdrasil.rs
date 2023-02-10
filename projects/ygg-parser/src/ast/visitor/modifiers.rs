use super::*;
use valkyrie_ast::{IdentifierPattern, ModifiedNode};

impl<'i> Extractor<Modified_identifierContextAll<'i>> for ModifiedNode<IdentifierNode> {
    fn take_one(node: &Modified_identifierContextAll<'i>) -> Option<Self> {
        let mods = IdentifierNode::take_many(&node.mods);
        let id = IdentifierNode::take(node.identifier.clone())?;
        Some(Self { base: id, terms: mods })
    }
}

impl<'i> Extractor<Modified_identifierContextAll<'i>> for IdentifierPattern {
    fn take_one(node: &Modified_identifierContextAll<'i>) -> Option<Self> {
        let mods = IdentifierNode::take_many(&node.mods);
        let id = IdentifierNode::take(node.id.clone())?;
        Some(Self { modifiers: ModifiersNode { terms: mods }, identifier: id })
    }
}
