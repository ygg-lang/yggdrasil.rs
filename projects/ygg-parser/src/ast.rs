use super::cst::{parse_namespace, YggdrasilType};
use std::ops::Range;
use yggdrasil_rt::{cst_mode::NodeID, AstNode, ConcreteContext, ParseState};

#[derive(Clone, Debug)]
pub struct NamepathNode {
    pub identifier: Vec<IdentifierNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct IdentifierNode {
    pub range: Range<usize>,
}

impl AstNode for NamepathNode {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Namespace;

    fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
}

impl AstNode for IdentifierNode {
    type NodeType = YggdrasilType;
    const KIND: Self::NodeType = YggdrasilType::Identifier;

    fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
}

#[test]
fn test() {
    let text = ParseState::new("a ::  b:: c");
    let mut ctx = ConcreteContext::<YggdrasilType>::default();
    let out = parse_namespace(text, &mut ctx);
    let root_id = out.as_result().unwrap().1;
    println!("{:#?}", ctx.get_typed(root_id));
    println!("{:#?}", ctx.filter_children(root_id, |_| true));
    // let out = NamepathNode::from_manager(&ctx, root_id);
    // println!("{:#?}", out);
}
