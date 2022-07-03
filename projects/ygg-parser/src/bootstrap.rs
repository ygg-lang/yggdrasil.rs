use std::hash::Hash;
use yggdrasil_rt::{CstContext, CstNode, LanguageID, LanguageManager, NodeID, NodeManager, NodeType, ParseResult, ParseState};

#[repr(i16)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Statement = 1,
    Expression = 2,
    Identifier = 3,
    IgnoredText = 9998,
    Ignored = 9999,
}

impl From<i16> for YggdrasilType {
    fn from(value: i16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<i16> for YggdrasilType {
    fn into(self) -> i16 {
        self as i16
    }
}

impl NodeType for YggdrasilType {
    fn get_language_id(&self) -> LanguageID {
        LanguageManager::id_from_name("Yggdrasil")
    }

    fn is_ignored(&self) -> bool {
        matches!(self, YggdrasilType::Ignored | YggdrasilType::IgnoredText)
    }
}

pub type ParseContext<'a> = CstContext<'a, YggdrasilType>;

pub fn parse_namespace<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, NodeID> {
    let id = ctx.random_scope();

    ctx.end_scope();
    todo!()
}

/// `a :: b :: c`
pub fn parse_namespace_aux1<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, ()> {
    let o = i;
    let (o, _) = o.match_fn(|i1| parse_identifier(i1, ctx))?;
    let (o, _) = o.match_fn(|i1| parse_ignored(i1, ctx))?;
    o.finish(())
}

pub fn parse_ignored<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, ()> {
    let this = ctx.random_id();
    let (o, b) = i.match_char_if(|c| c.is_whitespace(), "IGNORED")?;
    let node = CstNode::new(this).with_kind(YggdrasilType::Ignored);
    o.finish(())
}

pub fn parse_identifier<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, CstNode> {
    let id = ctx.random_id();
    let o = i;
    let (o, _) = o.match_str_if(|c| c.is_alphabetic(), "IDENTIFIER")?;
    o.finish(CstNode::new(id).with_kind(YggdrasilType::Identifier).with_range(i.start_offset, o.start_offset))
}

#[test]
fn test() {
    let text = ParseState::new("a::b::c");
    let manager = NodeManager::default();
    let mut ctx = ParseContext::new(&manager);
    let out = parse_identifier(text, &mut ctx);
    println!("{:?}", out);
}
