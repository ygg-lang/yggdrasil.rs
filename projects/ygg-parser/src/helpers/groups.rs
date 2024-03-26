use super::*;

impl<'i> AstBuilder<'i> for GroupStatementNode<'i> {
    type Output = ();

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let anno = TakeAnnotations { auto_tag: false, macros: self.decorator_call(), modifiers: self.modifier_call() }
            .build(ctx, state)?;
        let id = self.identifier().build(ctx, state).ok();
        let mut terms = vec![];
        for term in self.group_block().group_pair() {
            match term.build(ctx, state) {
                Ok(o) => terms.push(o.merge_annotations(anno.clone())),
                Err(_) => {}
            }
        }
        match id {
            Some(id) => {
                let mut name = vec![];
                for o in terms {
                    name.push(o.name.clone());
                    state.register(o)?
                }

                // terms.token_sets.insert(id.text.clone(), name);
            }
            None => {
                for o in terms {
                    state.register(o)?
                }
            }
        }

        Ok(())
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
