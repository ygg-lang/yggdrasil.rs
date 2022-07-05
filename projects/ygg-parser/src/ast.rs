use super::cst::{parse_namespace, ParseContext, YggdrasilType};
use std::ops::Range;
use yggdrasil_rt::{AstNode, CstContext, NodeID, ParseState};

#[derive(Clone, Debug)]
pub struct NamepathNode {
    pub identifier: Vec<IdentifierNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct IdentifierNode {
    pub range: Range<usize>,
}

impl NamepathNode {
    fn from_manager(ctx: &ParseContext, parent: NodeID) -> Self {
        let identifier = ctx.get_children(parent);
        let range = ctx.get_range(parent);
        Self { identifier, range }
    }
}

impl AstNode for NamepathNode {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Namespace;

    fn from_cst(ctx: &ParseContext, node: NodeID) -> Self {
        let identifier = ctx.get_children::<IdentifierNode>(node);
        let range = ctx.get_range(node);
        Self { identifier, range }
    }

    fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
}

impl AstNode for IdentifierNode {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Identifier;

    fn from_cst(ctx: &ParseContext, node: NodeID) -> Self {
        let leaf = ctx.get_node(&node).expect("Missing node");
        assert!(leaf.is_a(&[YggdrasilType::Identifier]));
        Self { range: leaf.get_range() }
    }

    fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
}

#[test]
fn test() {
    let text = ParseState::new("a ::  b:: c");
    let mut ctx = CstContext::<YggdrasilType>::default();
    let out = parse_namespace(text, &mut ctx);
    let root_id = out.as_result().unwrap().1;
    println!("{:#?}", ctx.get_typed(root_id));
    println!("{:#?}", ctx.filter_children(root_id, |_| true));
    let out = NamepathNode::from_manager(&ctx, root_id);
    println!("{:#?}", out);
}
