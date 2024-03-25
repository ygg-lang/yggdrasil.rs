use super::*;
use crate::{bootstrap::GroupBlockNode, helpers::annotations::TakeAnnotations};

impl<'i> AstBuilder<'i> for GroupStatementNode<'i> {
    type Output = (Option<YggdrasilIdentifier>, Vec<GrammarRule>);

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let anno = TakeAnnotations { auto_tag: false, macros: self.decorator_call(), modifiers: self.modifier_call() }
            .build(ctx, state)?;
        let name = self.identifier().build(ctx, state).ok();
        let mut out = vec![];
        for term in self.group_block().group_pair() {
            match term.build(ctx, state) {
                Ok(o) => out.push(o.merge_annotations(anno.clone())),
                Err(_) => {}
            }
        }
        Ok((name, out))
    }
}

impl<'i> AstBuilder<'i> for GroupPairNode<'i> {
    type Output = GrammarRule;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let name = self.identifier().build(ctx, state)?;
        let rule = GrammarRule {
            name,
            body: GrammarBody::Class { term: self.atomic().build(ctx, state)? },
            range: self.get_range(),
            ..Default::default()
        };
        Ok(rule)
    }
}
