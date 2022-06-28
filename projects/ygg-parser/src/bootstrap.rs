use peginator::{ParseResult, ParseState};
use std::marker::PhantomData;
use yggdrasil_rt::{CSTNode, LanguageID, NodeID, NodeManager, NodeType, ParseResult, ParseState};

#[repr(usize)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Statement = 1,
    Expression = 2,
    Identifier = 3,
}

impl From<usize> for YggdrasilType {
    fn from(value: usize) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<usize> for YggdrasilType {
    fn into(self) -> usize {
        self as usize
    }
}

impl NodeType for YggdrasilType {
    fn get_language_id(&self) -> LanguageID {
        todo!()
    }

    fn is_ignored(&self) -> bool {
        todo!()
    }
}

pub struct ParseContext<'m> {
    node_manager: &'m NodeManager,
    random: StdRng,
}

impl ParseContext<'_> {
    pub fn new(manager: &NodeManager) -> Self {
        Self { node_manager: manager, random: StdRng::from_entropy() }
    }
    pub fn random_id(&mut self) -> NodeID {
        self.random.gen()
    }
    pub fn parse_program(&mut self, i: ParseState) -> ParseResult<NodeID> {
        i.get_character()
    }
}

fn parse_program(i: ParseState) -> ParseResult<NodeID> {
    todo!()
}

fn parse_identifier(i: ParseState) -> ParseResult<NodeID> {
    todo!()
}
