use std::hash::Hash;
use yggdrasil_rt::{ConcreteNode, ConcreteTree, NodeId, NodeType, ParseResult, ParseState};

#[repr(u16)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Identifier = 10,
    Namespace = 100,
    WhiteSpace = 1000,
}

impl NodeType for YggdrasilType {
    fn is_ignored(&self) -> bool {
        todo!()
    }
}

pub struct CalculateCST {
    tree: ConcreteTree<YggdrasilType>,
}

impl CalculateCST {
    // namepath = identifier ~ ("::" ~ identifier)*
    fn parse_namepath<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, ()> {
        let namepath = self.tree.place_holder_node(parent, YggdrasilType::Namespace);
        match self.try_parse_namepath(state0, namepath) {
            ParseResult::Pending(state, data) => {
                self.tree.update_node(namepath, ConcreteNode::new(YggdrasilType::Namespace));
                ParseResult::Pending(state, data)
            }
            ParseResult::Stop(stop) => {
                self.tree.drop_node(namepath);
                ParseResult::Stop(stop)
            }
        }
    }
    fn try_parse_namepath<'a, 'b>(&'a mut self, state0: ParseState<'b>, namepath: NodeId) -> ParseResult<'b, ()> {
        // identifier
        let (state1, node1) = self.parse_identifier(state0, namepath)?;
        self.tree.append_node(namepath, node1);
        // ~
        let (state2, node2) = self.parse_ignore_space(state1, namepath)?;
        self.tree.append_node(namepath, node2);
        state2.finish(())
    }

    // identifier = [a-zA-Z][a-zA-Z0-9_]*
    fn parse_identifier<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let (state1, node) = state0 //
            .match_str_if(|c| c.is_alphabetic(), "Identifier")
            .map_inner(|_| ConcreteNode::new(YggdrasilType::Identifier))?;
        let id = node.append_to(&mut self.tree, parent);
        state1.finish(id)
    }
    // ignore_space = [ \t\n\r]*
    fn parse_ignore_space<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let (state1, node) = state0 //
            .match_str_if(|c| c.is_whitespace(), "IgnoreSpace")
            .map_inner(|_| ConcreteNode::new(YggdrasilType::WhiteSpace))?;
        let id = node.append_to(&mut self.tree, parent);
        state1.finish(id)
    }
}

#[test]
fn test() {
    let text = "1 + 1";
    let state = ParseState::new(text);
    let mut tree = CalculateCST { tree: ConcreteTree::<YggdrasilType>::new(text) };
    let root = tree.tree.create_root_node(ConcreteNode::new(YggdrasilType::Program));
    let result = tree.parse_identifier(state, root);
    println!("{:?}", result);
}
