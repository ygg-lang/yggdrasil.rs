use std::str::FromStr;

use yggdrasil_error::YggdrasilError;
use yggdrasil_parser::{
    bootstrap::{
        AtomicNode, ClassBlockNode, ClassStatementNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode,
        GrammarStatementNode, IdentifierNode, RootNode, StatementNode, TermNode, UnionStatementNode,
    },
    YggdrasilNode,
};

use crate::{
    grammar::GrammarInfo,
    nodes::YggdrasilExpression,
    rule::{GrammarBody, GrammarRule, YggdrasilIdentifier},
};

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
                StatementNode::ClassStatement(v) => match GrammarRule::build_class(v) {
                    Ok(_) => {}
                    Err(_) => {}
                },
                StatementNode::UnionStatement(v) => match GrammarRule::build_union(v) {
                    Ok(_) => {}
                    Err(_) => {}
                },
                StatementNode::GroupStatement(v) => {}
            }
        }
        Ok(out)
    }
}

impl GrammarInfo {
    fn visit_grammar(&mut self, node: &GrammarStatementNode) -> Result<(), YggdrasilError> {
        self.name = YggdrasilIdentifier::build(&node.identifier);

        Ok(())
    }
}

impl GrammarRule {
    fn build_class(node: &ClassStatementNode) -> Result<Self, YggdrasilError> {
        let name = YggdrasilIdentifier::build(&node.name);

        let node = match node.class_block.expression.expression_hard.as_slice() {
            [] => Err(YggdrasilError::syntax_error("empty class", node.get_range().unwrap_or_default()))?,
            [s @ ..] => Self {
                name,
                range: node.get_range().unwrap_or_default(),
                body: GrammarBody::Class { term: YggdrasilExpression::build_or(s)? },
                ..Default::default()
            },
        };
        Ok(node)
    }

    fn build_union(node: &UnionStatementNode) -> Result<Self, YggdrasilError> {
        todo!()
    }
}

impl YggdrasilExpression {
    fn build_or(node: &ExpressionNode) -> Result<Self, YggdrasilError> {
        match node.expression_hard.as_slice() {
            [head, rest @ ..] => {
                let head = YggdrasilExpression::build_hard(head)?;
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class", node.get_range().unwrap_or_default()))?,
        }
    }

    fn build_hard(node: &ExpressionHardNode) -> Result<Self, YggdrasilError> {
        match node.expression_soft.as_slice() {
            [head, rest @ ..] => {
                let head = YggdrasilExpression::build_soft(head)?;
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_soft(node: &ExpressionSoftNode) -> Result<Self, YggdrasilError> {
        match node.expression_tag.as_slice() {
            [head, rest @ ..] => {
                let head = YggdrasilExpression::build_tag_node(head)?;
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class", node.get_range().unwrap_or_default()))?,
        }
    }

    fn build_tag_node(node: &ExpressionTagNode) -> Result<Self, YggdrasilError> {
        match node.term.as_slice() {
            [head, rest @ ..] => {
                let head = YggdrasilExpression::build_term(head)?;

                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_term(node: &TermNode) -> Result<Self, YggdrasilError> {
        YggdrasilExpression::build_atomic(&node.atomic)
    }
    fn build_atomic(node: &AtomicNode) -> Result<Self, YggdrasilError> {
        match node {
            AtomicNode::Atomic0(_) => {
                todo!()
            }
            AtomicNode::Boolean(_) => {
                todo!()
            }
            AtomicNode::FunctionCall(_) => {
                todo!()
            }
            AtomicNode::Identifier(_) => {
                todo!()
            }
            AtomicNode::RegexEmbed(_) => {
                todo!()
            }
            AtomicNode::RegexRange(_) => {
                todo!()
            }
            AtomicNode::String(_) => {
                todo!()
            }
        }
    }
}

impl YggdrasilIdentifier {
    fn build(node: &IdentifierNode) -> Self {
        Self { text: node.text.clone(), range: node.get_range().unwrap_or_default() }
    }
}
