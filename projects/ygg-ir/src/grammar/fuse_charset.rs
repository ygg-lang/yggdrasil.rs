use yggdrasil_error::{Diagnostic, YggdrasilResult};

use crate::{ChoiceExpression, CodeOptimizer, ExpressionKind, GrammarInfo};

pub struct FuseCharset {}

impl CodeOptimizer for FuseCharset {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo> {
        let _ = info;
        self.fuse_choice(todo!())?;
        todo!()
    }
}

impl FuseCharset {
    fn fuse_choice(&mut self, choice: &ChoiceExpression) -> YggdrasilResult<ChoiceExpression> {
        let mut errors = vec![];
        for branch in &choice.branches {
            match &branch.kind {
                ExpressionKind::Choice(_) => {}
                ExpressionKind::Concat(_) => {}
                ExpressionKind::Unary(_) => {}
                ExpressionKind::Rule(_) => {}
                ExpressionKind::Data(_) => todo!(),
                ExpressionKind::Function(_) => {}
            }
        }
        Ok(Diagnostic { success: ChoiceExpression { branches: Default::default() }, errors })
    }
}