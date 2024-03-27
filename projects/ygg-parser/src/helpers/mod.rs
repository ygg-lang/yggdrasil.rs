mod annotations;
mod classes;
mod expressions;
mod groups;
mod regex;
mod strings;
mod unions;

use crate::{
    bootstrap::{
        AtomicNode, BooleanNode, BranchTagNode, CallBodyNode, ClassBlockNode, ClassStatementNode, DecoratorCallNode,
        EscapedCharacterNode, EscapedUnicodeNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode,
        GrammarStatementNode, GroupPairNode, GroupStatementNode, IdentifierNode, ModifierCallNode, PrefixNode, RangeExactNode,
        RangeNode, RegexEmbedNode, RegexItemNode, RootNode, StatementNode, StringItemNode, StringNormalNode, SuffixNode,
        TermNode, UnionBlockNode, UnionBranchNode, UnionStatementNode,
    },
    helpers::annotations::TakeAnnotations,
    states::{ParseContext, ParseState},
    traits::AstBuilder,
};
use std::borrow::Cow;
use yggdrasil_error::{Failure, FileCache, FileID, Result, Success, Validation, YggdrasilError};
use yggdrasil_ir::{
    data::{YggdrasilRegex, YggdrasilText},
    grammar::GrammarInfo,
    nodes::{UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{BigInt, GrammarAtomic, GrammarBody, GrammarRule, Num, YggdrasilBranch, YggdrasilCounter, YggdrasilIdentifier},
};
use yggdrasil_rt::YggdrasilNode;

pub fn parse_grammar_info(cache: &mut FileCache, id: FileID) -> Validation<GrammarInfo> {
    let text = match cache.fetch(&id) {
        Ok(o) => o.to_string(),
        Err(e) => Err(YggdrasilError::from(e))?,
    };
    let root = match RootNode::from_str(&text, 0) {
        Ok(o) => o,
        Err(e) => Err(YggdrasilError::from(e).with_file(id))?,
    };
    let mut state = ParseState::default();
    let ctx = ParseContext { file: id };
    match root.build(&ctx, &mut state) {
        Ok(value) => Success { value, diagnostics: state.get_errors() },
        Err(fatal) => Failure { fatal, diagnostics: state.get_errors() },
    }
}

impl<'i> AstBuilder<'i> for RootNode<'i> {
    type Output = GrammarInfo;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        for s in self.statement() {
            match s {
                StatementNode::GrammarStatement(v) => v.build(ctx, state)?,
                StatementNode::ClassStatement(v) => v.build(ctx, state)?,
                StatementNode::UnionStatement(v) => v.build(ctx, state)?,
                StatementNode::GroupStatement(v) => v.build(ctx, state)?,
            }
        }
        Ok(state.get_grammar())
    }
}

impl<'i> AstBuilder<'i> for GrammarStatementNode<'i> {
    type Output = ();

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let new = self.identifier().build(ctx, state)?;

        Ok(state.rename(new))
    }
}

impl<'i> AstBuilder<'i> for Option<IdentifierNode<'i>> {
    type Output = YggdrasilIdentifier;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        match self {
            Some(s) => s.build(ctx, state),
            None => Err(YggdrasilError::runtime_error("todo")),
        }
    }
}

impl<'i> AstBuilder<'i> for IdentifierNode<'i> {
    type Output = YggdrasilIdentifier;

    fn build(&self, ctx: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        Ok(YggdrasilIdentifier { text: self.get_str().to_string(), span: ctx.file.with_range(self.get_range()) })
    }
}
