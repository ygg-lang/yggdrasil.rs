use std::str::FromStr;

use yggdrasil_error::YggdrasilError;
use yggdrasil_parser::{
    bootstrap::{
        AtomicNode, BooleanNode, ClassStatementNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode,
        GrammarStatementNode, IdentifierNode, RootNode, StatementNode, StringNode, TermNode, UnionBlockNode, UnionBranchNode,
        UnionStatementNode,
    },
    YggdrasilNode,
};

use crate::{
    data::{YggdrasilRegex, YggdrasilText},
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
                    Err(e) => {
                        println!("{e:?}");
                        println!("Class: {}", v.class_block.expression.text);
                    }
                },
                StatementNode::UnionStatement(v) => match GrammarRule::build_union(v) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{e:?}");
                        for i in &v.union_block.union_branch {
                            println!("Union: {}", i.expression.text);
                        }
                    }
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
        let rule = Self {
            name,
            body: GrammarBody::Class { term: YggdrasilExpression::build_or(&node.class_block.expression)? },
            range: node.get_range().unwrap_or_default(),
            ..Default::default()
        };
        Ok(rule)
    }

    fn build_union(node: &UnionStatementNode) -> Result<Self, YggdrasilError> {
        let name = YggdrasilIdentifier::build(&node.name);
        let rule =
            Self { name, body: GrammarBody::Union { branches: vec![] }, range: node.get_range().unwrap_or_default(), ..Default::default() };
        Ok(rule)
    }
}

impl YggdrasilExpression {
    fn build_or(node: &ExpressionNode) -> Result<Self, YggdrasilError> {
        match node.expression_hard.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_hard(head)?;
                for term in rest {
                    head |= YggdrasilExpression::build_hard(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class or", node.get_range().unwrap_or_default()))?,
        }
    }

    fn build_hard(node: &ExpressionHardNode) -> Result<Self, YggdrasilError> {
        match node.expression_soft.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_soft(head)?;
                for term in rest {
                    head += YggdrasilExpression::build_soft(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class hard", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_soft(node: &ExpressionSoftNode) -> Result<Self, YggdrasilError> {
        match node.expression_tag.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_tag_node(head)?;
                for term in rest {
                    head &= YggdrasilExpression::build_tag_node(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class soft", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_tag_branch(node: &UnionBranchNode) -> Result<Vec<Self>, YggdrasilError> {
        todo!()
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
        let expr = match node {
            AtomicNode::Atomic0(e) => YggdrasilExpression::build_or(e)?,
            AtomicNode::Boolean(v) => match v {
                BooleanNode::Boolean0 => YggdrasilExpression::boolean(true),
                BooleanNode::Boolean1 => YggdrasilExpression::boolean(true),
            },
            AtomicNode::FunctionCall(_) => {
                todo!()
            }
            AtomicNode::Identifier(v) => YggdrasilIdentifier::build(v).into(),
            AtomicNode::RegexEmbed(v) => YggdrasilRegex::new(v.text.trim_matches('/'), v.get_range().unwrap_or_default()).into(),
            AtomicNode::RegexRange(v) => YggdrasilRegex::new(&v.text, v.get_range().unwrap_or_default()).into(),
            AtomicNode::String(v) => match v {
                StringNode::String0(s) => YggdrasilText::new(s.trim_matches('\''), Default::default()).into(),
                StringNode::String1(s) => YggdrasilText::new(s.trim_matches('"'), Default::default()).into(),
            },
        };
        Ok(expr)
    }
}

impl YggdrasilIdentifier {
    fn build(node: &IdentifierNode) -> Self {
        Self { text: node.text.clone(), range: node.get_range().unwrap_or_default() }
    }
}
