use super::*;
use crate::{bootstrap::ClassBlockNode, helpers::annotations::TakeAnnotations};
use yggdrasil_ir::rule::GrammarRuleAttributes;

impl<'i> AstBuilder<'i> for ClassStatementNode<'i> {
    type Output = ();

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let attributes = TakeAnnotations {
            auto_tag: self.op_remark().is_none(),
            macros: self.decorator_call(),
            modifiers: self.modifier_call(),
        }
        .build(ctx, state)?;
        let rule = GrammarRule {
            attributes,
            name: self.name().build(ctx, state)?,
            body: self.class_block().build(ctx, state)?,
            range: self.get_range(),
            ..Default::default()
        };
        state.register(rule)
    }
}
impl<'i> AstBuilder<'i> for ClassBlockNode<'i> {
    type Output = GrammarBody;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        Ok(GrammarBody::Class { term: self.expression().build(ctx, state)? })
    }
}
