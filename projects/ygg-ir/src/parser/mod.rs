use std::str::FromStr;

use yggdrasil_error::YggdrasilError;
use yggdrasil_parser::{
    bootstrap::{GrammarStatementNode, IdentifierNode, RootNode, StatementNode},
    YggdrasilNode,
};

use crate::{grammar::GrammarInfo, rule::YggdrasilIdentifier};

impl FromStr for GrammarInfo {
    type Err = YggdrasilError;

    fn from_str(s: &str) -> Result<Self, YggdrasilError> {
        GrammarInfo::try_from(RootNode::from_str(s)?)
    }
}

impl TryFrom<RootNode> for GrammarInfo {
    type Error = YggdrasilError;

    fn try_from(value: RootNode) -> Result<Self, Self::Error> {
        let mut out = GrammarInfo::default();
        for s in &value.statement {
            match s {
                StatementNode::GrammarStatement(v) => out.visit_grammar(v)?,
                StatementNode::ClassStatement(v) => {}
                StatementNode::GroupStatement(v) => {}
                StatementNode::UnionStatement(v) => {}
            }
        }
        Ok(out)
    }
}

impl YggdrasilIdentifier {
    fn build(node: &IdentifierNode) -> Self {
        Self { text: node.text.clone(), range: node.get_range().unwrap_or_default() }
    }
}

impl GrammarInfo {
    fn visit_grammar(&mut self, node: &GrammarStatementNode) -> Result<(), YggdrasilError> {
        self.name = YggdrasilIdentifier::build(&node.identifier);

        Ok(())
    }
}
