use yggdrasil_rt::{
    ast_mode::{Parsed, YResult, YState},
    results::YError,
    CSTNode, NodeType,
};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TNodes {
    A,
    B,
    C,
}

#[derive(Debug)]
struct Output1 {
    a: char,
    b: Vec<char>,
    c: char,
}

impl Into<usize> for TNodes {
    fn into(self) -> usize {
        self as usize
    }
}

impl NodeType for TNodes {}

/// `ab{1,3}c`
fn parse_output1(state: CState) -> YResult<Output1> {
    let Parsed(a, state) = state.parse_char('a')?;
    let Parsed(b, state) = state.parse_repeats(1, 3, |state| state.parse_char('b'))?;
    let Parsed(c, state) = state.parse_char('c')?;
    Parsed::ok(Output1 { a, b, c }, state)
}

#[test]
fn test() {
    let node = CSTNode::new(0).with_range(0, 1).with_kind(TNodes::GrammarInfo);
    let ids = node.collect_ids();
    assert_eq!(ids, vec![0]);
}
