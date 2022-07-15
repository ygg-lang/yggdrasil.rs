use super::*;
use yggdrasil_rt::helpers::dec_str;

impl YggdrasilCST {
    /// id | num
    pub fn parse_value<'a, 'b>(&'a mut self, state0: ParseState<'b>, this: NodeId) -> ParseResult<'b, NodeId> {
        let (state1, id) = state0
            .begin_choice()
            .or_else(|s| self.parse_namepath(s, this))
            .or_else(|s| self.parse_number(s, this))
            .end_choice()?;
        state1.finish(id)
    }
    /// @transaction [parse_namepath_0]()
    pub fn parse_namepath<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let this = self.tree.place_holder_node(parent);
        self.parse_namepath_0(state0, this)
            .dispatch(
                |s| self.tree.update_node(this, ConcreteNode::new(YggdrasilType::Namespace).with_offset(state0, s)),
                |_| self.tree.drop_node(this),
            )
            .map_inner(|_| this)
    }
    /// [identifier](YggdrasilCST::parse_identifier) [~](YggdrasilCST::parse_ignore_space) [parse_namepath_1](YggdrasilCST::parse_namepath_1)*
    pub fn parse_namepath_0<'a, 'b>(&'a mut self, state0: ParseState<'b>, this: NodeId) -> ParseResult<'b, ()> {
        let (state1, _) = state0.match_fn(|s| self.parse_identifier(s, this))?;
        let (state2, _) = state1.match_optional(|s| self.parse_ignore_space(s, this))?;
        let (state3, _) = state2.match_repeats(|state| self.parse_namepath_1(state, this))?;
        state3.finish(())
    }
    /// `::` [~]() [identifier](YggdrasilCST::parse_identifier) [~]()
    pub fn parse_namepath_1<'a, 'b>(&'a mut self, state0: ParseState<'b>, this: NodeId) -> ParseResult<'b, ()> {
        let (state1, _) = state0.match_fn(|s| self.parse_identifier(s, this))?;
        let (state2, _) = state1.match_optional(|s| self.parse_ignore_space(s, this))?;
        let (state3, _) = state2.match_fn(|s| self.parse_identifier(s, this))?;
        let (state4, _) = state3.match_optional(|s| self.parse_ignore_space(s, this))?;
        state4.finish(())
    }
}

impl YggdrasilCST {
    /// `[a-zA-Z][a-zA-Z0-9_]*`
    pub fn parse_identifier<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let (state1, node) = state0 //
            .match_str_if(|c| c.is_alphabetic(), "Identifier")
            .map_inner(|_| ConcreteNode::new(YggdrasilType::Identifier))?;
        let this = self.tree.create_node(node.with_offset(state0, state1));
        self.tree.append_node(parent, this);
        state1.finish(this)
    }
    pub fn parse_ignore_text<'a, 'b>(
        &'a mut self,
        state0: ParseState<'b>,
        parent: NodeId,
        text: &'static str,
    ) -> ParseResult<'b, NodeId> {
        let (state1, node) = state0 //
            .match_str(text)
            .map_inner(|_| ConcreteNode::new(YggdrasilType::Literal))?;
        let this = self.tree.create_node(node.with_offset(state0, state1));
        self.tree.append_node(parent, this);
        state1.finish(this)
    }
    pub fn parse_number<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let (state1, node) = state0 //
            .match_fn(dec_str)
            .map_inner(|_| ConcreteNode::new(YggdrasilType::Number))?;
        let this = self.tree.create_node(node.with_offset(state0, state1));
        self.tree.append_node(parent, this);
        state1.finish(this)
    }
    // ignore_space = [ \t\n\r]*
    pub fn parse_ignore_space<'a, 'b>(&'a mut self, state0: ParseState<'b>, parent: NodeId) -> ParseResult<'b, NodeId> {
        let (state1, node) = state0 //
            .match_str_if(|c| c.is_whitespace(), "IgnoreSpace")
            .map_inner(|_| ConcreteNode::new(YggdrasilType::WhiteSpace))?;
        let this = self.tree.create_node(node.with_offset(state0, state1));
        self.tree.append_node(parent, this);
        state1.finish(this)
    }
}
