use super::*;
use crate::helpers::annotations::TakeAnnotations;

impl<'i> AstBuilder<'i> for UnionStatementNode<'i> {
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
            body: self.union_block().build(ctx, state)?,
            range: self.get_range(),
            ..Default::default()
        };
        state.register(rule)
    }
}
impl<'i> AstBuilder<'i> for UnionBlockNode<'i> {
    type Output = GrammarBody;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let raw = self.union_branch();
        let mut branches = Vec::with_capacity(raw.len());
        for branch in raw {
            branches.push(branch.build(ctx, state)?)
        }
        Ok(GrammarBody::Union { branches, refined: Default::default() })
    }
}

impl<'i> AstBuilder<'i> for UnionBranchNode<'i> {
    type Output = YggdrasilVariant;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        Ok(YggdrasilVariant {
            tag: self.branch_tag().build(ctx, state).ok(),
            branch: self.expression_hard().build(ctx, state)?,
        })
    }
}
impl<'i> AstBuilder<'i> for Option<BranchTagNode<'i>> {
    type Output = YggdrasilIdentifier;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        match self {
            Some(s) => s.identifier().build(ctx, state),
            None => Err(YggdrasilError::runtime_error("todo")),
        }
    }
}
