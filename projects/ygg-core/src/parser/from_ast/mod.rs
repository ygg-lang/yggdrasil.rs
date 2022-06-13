use yggdrasil_ir::{ExpressionNode, QErrorKind, SyntaxError};
use yggdrasil_parser::RuleBodyNode;

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
        let mut rule = GrammarRule::new(&node.identifier.string, &node.position);
        for modifier in &node.modifiers.items {
            match modifier.string.as_str() {
                "entry" => rule.entry = true,
                _ => {
                    self.syntax_error(format!("Unknown modifier `{}`", modifier.string), &modifier.position);
                }
            }
        }
        let expr = self.visit_rule_body(&node.rule_body);
    }
    pub fn visit_rule_body(&mut self, node: &RuleBodyNode) -> ExpressionNode {
        for rule in &node.rules {
            self.visit_rule(rule);
        }
    }
}
