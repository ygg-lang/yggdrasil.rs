mod annotations;
mod classes;
mod expressions;
mod unions;

use crate::{
    bootstrap::{
        AtomicNode, BooleanNode, BranchTagNode, CallBodyNode, ClassStatementNode, DecoratorCallNode, EscapedUnicodeNode,
        ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode, GrammarStatementNode, GroupPairNode,
        GroupStatementNode, IdentifierNode, ModifierCallNode, PrefixNode, RegexEmbedNode, RegexItemNode, RootNode,
        StatementNode, StringItemNode, SuffixNode, TermNode, UnionBlockNode, UnionBranchNode, UnionStatementNode,
    },
    states::{ParseContext, ParseState},
    traits::AstBuilder,
};
use std::fmt::{Display, Formatter, Write};
use yggdrasil_error::{Failure, FileCache, FileID, Result, Success, Validation, YggdrasilError};
use yggdrasil_ir::{
    data::{YggdrasilRegex, YggdrasilText},
    grammar::GrammarInfo,
    nodes::{UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{BigInt, GrammarAtomic, GrammarBody, GrammarRule, Num, YggdrasilCounter, YggdrasilIdentifier, YggdrasilVariant},
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
    let ctx = ParseContext { id };
    match root.build(&ctx, &mut state) {
        Ok(value) => Success { value, diagnostics: state.get_errors() },
        Err(fatal) => Failure { fatal, diagnostics: state.get_errors() },
    }
}

impl<'i> AstBuilder<'i> for RootNode<'i> {
    type Output = GrammarInfo;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let mut out = GrammarInfo::default();
        for s in self.statement() {
            match s {
                StatementNode::GrammarStatement(v) => v.build(ctx, state)?,
                StatementNode::ClassStatement(v) => v.build(ctx, state)?,
                StatementNode::UnionStatement(v) => v.build(ctx, state)?,
                StatementNode::GroupStatement(v) => match v.build(ctx, state) {
                    Ok((id, terms)) => match id {
                        Some(id) => {
                            let mut name = vec![];
                            for o in terms {
                                name.push(o.name.clone());
                                out.rules.insert(o.name.text.clone(), o);
                            }
                            out.token_sets.insert(id.text.clone(), name);
                        }
                        None => {
                            for o in terms {
                                out.rules.insert(o.name.text.clone(), o);
                            }
                        }
                    },
                    Err(e) => state.add_error(e),
                },
            }
        }
        Ok(out)
    }
}

impl<'i> AstBuilder<'i> for GrammarStatementNode<'i> {
    type Output = ();

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let new = self.identifier().build(ctx, state)?;

        Ok(state.rename(new))
    }
}

impl<'i> AstBuilder<'i> for GroupStatementNode<'i> {
    type Output = (Option<YggdrasilIdentifier>, Vec<GrammarRule>);

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let name = match self.identifier() {
            Some(s) => Some(s.build(ctx, state)?),
            None => None,
        };
        let mut out = vec![];
        for term in self.group_block().group_pair() {
            match term.build(ctx, state) {
                Ok(o) => out.push(o.with_annotations(self.annotations().build(ctx, state)?)),
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

impl<'i> AstBuilder<'i> for IdentifierNode<'i> {
    type Output = YggdrasilIdentifier;

    fn build(&self, ctx: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        Ok(YggdrasilIdentifier { text: self.get_str().to_string(), span: ctx.id.with_range(self.get_range()) })
    }
}

impl<'i> EscapedUnicodeNode<'i> {
    pub fn as_char(&self) -> Option<char> {
        let u = u32::from_str_radix("ABC", 16).ok()?;
        char::from_u32(u)
    }
}

impl<'i> Display for RegexEmbedNode<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in &self.regex_item() {
            match item {
                RegexItemNode::EscapedCharacter(v) => match v.get_chars().last() {
                    Some(c) => match c {
                        's' => f.write_str("\\s")?,
                        'n' => f.write_str("\\n")?,
                        'r' => f.write_str("\\r")?,
                        'd' => f.write_str("\\d")?,
                        'p' => f.write_str("\\p")?,
                        c @ ('(' | ')' | '[' | ']' | '{' | '}') => {
                            f.write_char('\\')?;
                            f.write_char(c)?;
                        }
                        '.' => f.write_str("\\.")?,
                        _ => f.write_char(c)?,
                    },
                    None => {}
                },
                RegexItemNode::RegexCharacter(v) => f.write_str(v.get_str())?,
            }
        }
        Ok(())
    }
}
