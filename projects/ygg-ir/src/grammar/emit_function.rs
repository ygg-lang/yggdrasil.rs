use crate::{CodeOptimizer, ExpressionKind, ExpressionNode, GrammarInfo};
use yggdrasil_error::YggdrasilResult;

pub struct EmitFunction {}

impl CodeOptimizer for EmitFunction {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo> {
        todo!()
    }
}

impl EmitFunction {
    fn emit_function(&mut self, e: &ExpressionNode) -> ExpressionNode {
        match e.kind {
            ExpressionKind::Function(_) => {}
            ExpressionKind::Choice(_) => {}
            ExpressionKind::Concat(_) => {}
            ExpressionKind::Unary(_) => {}
            ExpressionKind::Rule(_) => {}
            ExpressionKind::Data(_) => {}
        }

        ExpressionNode { kind: (), branch_tag: "".to_string(), node_tag: "".to_string() }
    }
}

impl ExpressionNode {}

impl FunctionExpression {
    pub fn emit_builtin(self) -> ExpressionNode {}
}
