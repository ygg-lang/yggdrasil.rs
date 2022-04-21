use crate::{ChoiceExpression, CodeOptimizer, DataKind, ExpressionKind, GrammarInfo};
use yggdrasil_error::{Diagnostic, YggdrasilResult};

pub struct FuseCharset {}

impl CodeOptimizer for FuseCharset {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo> {
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
                ExpressionKind::Data(data) => match &**data {
                    DataKind::Integer(_) => {}
                    DataKind::String(_) => {}
                    DataKind::CharacterAny => {}
                    DataKind::Character(_) => {}
                    DataKind::CharacterBuiltin(_) => {}
                    DataKind::CharacterRange(_) => {}
                    DataKind::CharacterSet(_) => {}
                },
            }
        }
        Ok(Diagnostic { success: ChoiceExpression { branches: Default::default() }, errors })
    }
}
