use yggdrasil_ir::GrammarInfo;
use yggdrasil_parser::{DefineStatement, ProgramNode, StatementNode};

struct YggParser {
    out: GrammarInfo,
}

impl YggParser {
    pub fn visit_program(&mut self, node: ProgramNode) {
        for statement in &node.statements {
            match statement {
                StatementNode::DefineStatement(d) => self.visit_class_statement(d),
                StatementNode::EmptyStatement(_) => {}
            }
        }
    }
    pub fn visit_class_statement(&mut self, node: &DefineStatement) {
        for modifier in &node.modifiers.items {
            self.visit_identifier(modifier);
        }
        self.visit_identifier(&node.identifier);
        if let Some(out_type) = &node.out_type {
            self.visit_out_type(out_type);
        }
        self.visit_rule_body(&node.rule_body);
    }
}
