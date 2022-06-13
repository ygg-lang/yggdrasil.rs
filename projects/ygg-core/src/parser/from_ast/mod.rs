use yggdrasil_ir::{ExpressionNode, GrammarRuleKind, QErrorKind, SyntaxError};
use yggdrasil_parser::{ExprNode, IdentifierNode, RuleBodyNode};

use super::*;

impl ParseContext {
    #[inline]
    pub fn syntax_error<S>(&mut self, message: S, position: &Range<usize>)
    where
        S: Into<String>,
    {
        let kind = SyntaxError { message: message.into(), file: Default::default(), span: position.clone() };
        let error = QError { error: Box::new(QErrorKind::Syntax(kind)), level: Default::default(), source: None };
        self.errors.push(error);
    }
}

impl ParseContext {
    pub fn visit_program(&mut self, node: ProgramNode) {
        for statement in &node.statement {
            match statement {
                StatementNode::Class(v) => self.visit_class_statement(v),
            }
        }
    }
    pub fn visit_class_statement(&mut self, node: &ClassStatementNode) {
        let mut rule = GrammarRule::new(&node.identifier.string, &node.position, GrammarRuleKind::Class);
        let mut atomic = false;
        for modifier in &node.modifiers.items {
            match modifier.string.as_str() {
                "entry" => rule.entry = true,
                "atomic" => atomic = true,
                _ => {
                    self.syntax_error(format!("Unknown modifier `{}`", modifier.string), &modifier.position);
                }
            }
        }
        let expr = self.visit_rule_body(&node.rule_body, true);
        rule.body = expr;
        self.out.rules.insert(rule.name.clone(), rule);
    }
    pub fn visit_rule_body(&mut self, node: &RuleBodyNode, atomic: bool) -> ExpressionNode {
        self.visit_expr(&node.expr, atomic)
    }
    pub fn visit_expr(&mut self, node: &ExprNode, atomic: bool) -> ExpressionNode {
        match node {
            ExprNode::Choice { lhs, rhs } => self.visit_expr(lhs, atomic) | self.visit_expr(rhs, atomic),
            ExprNode::SoftConcat { lhs, rhs } => self.visit_expr(lhs, atomic) + self.visit_expr(rhs, atomic),
            ExprNode::Concat { lhs, rhs } if atomic => self.visit_expr(lhs, atomic) & self.visit_expr(rhs, atomic),
            ExprNode::Concat { lhs, rhs } => self.visit_expr(lhs, atomic) + self.visit_expr(rhs, atomic),
            ExprNode::Suffix { expr, suffix } => {
                todo!()
            }
            ExprNode::MarkTag { lhs, rhs } => {
                let expr = self.visit_expr(rhs, atomic);
                expr.with_tag(&lhs.string)
            }
            ExprNode::Negative { expr } => {
                todo!()
            }
            ExprNode::Group { expr } => self.visit_expr(expr, atomic),
            ExprNode::Identifier { identifier } => {
                todo!()
            }
        }
    }

    pub fn visit_identifier(&mut self, node: &IdentifierNode) -> String {
        node.string.clone()
    }
}
