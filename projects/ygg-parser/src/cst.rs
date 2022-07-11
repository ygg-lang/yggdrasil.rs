use std::{hash::Hash, ops::Range};
use yggdrasil_rt::{ConcreteNode, ConcreteTree, NodeId, NodeType, ParseResult, ParseState};

#[repr(i16)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Identifier = 10,
    Namespace = 100,
    WhiteSpace = 1000,
    Literal = 11111,
    Missing = 666,
}

impl Default for YggdrasilType {
    fn default() -> Self {
        YggdrasilType::Missing
    }
}

impl NodeType for YggdrasilType {
    fn is_ignored(&self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct YggdrasilNamespace {
    #[cfg(debug_assertions)]
    pub text: String,
    pub identifiers: Vec<YggdrasilIdentifier>,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct YggdrasilIdentifier {
    #[cfg(debug_assertions)]
    pub text: String,
    pub range: Range<usize>,
}

pub struct CalculateCST {
    tree: ConcreteTree<YggdrasilType>,
}

impl CalculateCST {
    // namepath = identifier ~ ("::" ~ identifier ~)*
    fn parse_namepath<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let this = self.tree.place_holder_node(parent);
        self.try_parse_namepath(state0, this).dispatch(
            |s| self.tree.update_node(this, ConcreteNode::new(YggdrasilType::Namespace).with_offset(state0, s)),
            |_| self.tree.drop_node(this),
        )
    }
    fn try_parse_namepath<'a, 'b>(&'a mut self, state0: ParseState<'b>, this: NodeId) -> ParseResult<'b, NodeId> {
        // identifier
        let (state1, _) = self.parse_identifier(state0, this)?;
        // ~
        let (state2, _) = state1.match_optional(|s| self.parse_ignore_space(s, this))?;
        // ("::" ~ identifier ~)*
        let (state3, _) = state2.match_repeats(|state| self.parse_namepath_1(state, this))?;
        state3.finish(this)
    }
    fn parse_namepath_1<'a, 'b>(&'a mut self, state0: ParseState<'b>, this: NodeId) -> ParseResult<'b, ()> {
        // "::"
        let (state1, _) = self.parse_ignore_text(state0, this, "::")?;
        // ~
        let (state2, _) = state1.match_optional(|s| self.parse_ignore_space(s, this))?;
        // identifier
        let (state3, _) = self.parse_identifier(state2, this)?;
        // ~
        let (state4, _) = state3.match_optional(|s| self.parse_ignore_space(s, this))?;
        state4.finish(())
        // identifier
    }
}

impl CalculateCST {
    fn extract_namespace(&self, id: NodeId) -> Option<YggdrasilNamespace> {
        let cst = self.tree.get_node(id)?;
        debug_assert!(cst.kind == YggdrasilType::Namespace, "Expected `YggdrasilType::Namespace`, got `{:?}`", cst.kind);
        let mut identifiers = Vec::new();
        for (id, child) in self.tree.children(id) {
            if child.kind == YggdrasilType::Namespace {
                match self.extract_identifier(id) {
                    Some(s) => {
                        identifiers.push(s);
                    }
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
    fn extract_identifier(&self, id: NodeId) -> Option<YggdrasilIdentifier> {
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

impl CalculateCST {
    // identifier = [a-zA-Z][a-zA-Z0-9_]*
    fn parse_identifier<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, ()> {
        let (state1, node) = state0 //
            .match_str_if(|c| c.is_alphabetic(), "Identifier")
            .map_inner(|_| ConcreteNode::new(YggdrasilType::Identifier))?;
        let this = self.tree.create_node(node);
        self.tree.append_node(parent, this);
        state1.finish(())
    }
    fn parse_ignore_text<'a, 'b>(
        &'a mut self,
        state0: ParseState<'b>,
        parent: NodeId,
        text: &'static str,
    ) -> ParseResult<'b, ()> {
        let (state1, node) = state0 //
            .match_str(text)
            .map_inner(|_| ConcreteNode::new(YggdrasilType::Literal))?;
        let this = self.tree.create_node(node);
        self.tree.append_node(parent, this);
        state1.finish(())
    }

    // ignore_space = [ \t\n\r]*
    fn parse_ignore_space<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, ()> {
        let (state1, node) = state0 //
            .match_str_if(|c| c.is_whitespace(), "IgnoreSpace")
            .map_inner(|_| ConcreteNode::new(YggdrasilType::WhiteSpace))?;
        let this = self.tree.create_node(node);
        self.tree.append_node(parent, this);
        state1.finish(())
    }
}

#[test]
fn test() {
    let text = "c::i + j";
    let state = ParseState::new(text);
    let mut tree = CalculateCST { tree: ConcreteTree::<YggdrasilType>::new(text) };
    let root = tree.tree.create_node(ConcreteNode::new(YggdrasilType::Program));
    let result = tree.parse_namepath(state, root).as_result().unwrap();
    println!("{}", tree.tree);
    println!("{:#?}", result);
    let ns = tree.extract_namespace(result.1);
    println!("{:#?}", ns);
}
