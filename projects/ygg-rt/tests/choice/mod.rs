use yggdrasil_rt::CSTNode;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum YggNodes {
    GrammarInfo,
    GrammarRule,
    ExpressionNode,
    ExpressionKind,
    Operator,
    RuleReference,
    UnaryExpression,
    ConcatExpression,
    ChoiceExpression,
    DataKind,
}

impl Into<usize> for YggNodes {
    fn into(self) -> usize {
        self as usize
    }
}

impl NodeType for YggNodes {}

#[test]
fn test() {
    let node = CSTNode::new(0).with_range(0, 1).with_kind(YggNodes::GrammarInfo);
    let ids = node.collect_ids();
    assert_eq!(ids, vec![0]);
}
