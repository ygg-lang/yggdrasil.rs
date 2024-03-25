mod annotations;
mod expressions;
use crate::{
    bootstrap::{
        AtomicNode, BooleanNode, CallBodyNode, ClassStatementNode, DecoratorCallNode, EscapedUnicodeNode, ExpressionHardNode,
        ExpressionNode, ExpressionSoftNode, ExpressionTagNode, GrammarStatementNode, GroupPairNode, GroupStatementNode,
        IdentifierNode, ModifierCallNode, PrefixNode, RegexEmbedNode, RegexItemNode, RootNode, StatementNode, StringItemNode,
        SuffixNode, TermNode, UnionBranchNode, UnionStatementNode,
    },
    states::{ParseContext, ParseState},
    traits::{AstBuilder, WithAnnotation},
};
use indexmap::IndexMap;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
    ops::Range,
};
use yggdrasil_error::{Failure, FileCache, FileID, Result, Success, Validation, YggdrasilError};
use yggdrasil_ir::{
    data::{YggdrasilRegex, YggdrasilText},
    grammar::GrammarInfo,
    nodes::{UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{BigInt, GrammarAtomic, GrammarBody, GrammarRule, Num, YggdrasilCounter, YggdrasilIdentifier},
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
                StatementNode::ClassStatement(v) => match v.build(ctx, state) {
                    Ok(o) => {
                        out.rules.insert(o.name.text.clone(), o);
                    }
                    Err(e) => state.add_error(e),
                },
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
        state.grammar.name = self.identifier().build(ctx, state)?;
        Ok(())
    }
}

impl<'i> AstBuilder<'i> for ClassStatementNode<'i> {
    type Output = GrammarRule;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let name = self.name().build(ctx, state)?;
        let rule = GrammarRule {
            name,
            body: GrammarBody::Class { term: self.class_block().expression().build(ctx, state)? },
            range: self.get_range(),
            ..Default::default()
        }
        .with_annotation(self.annotations());
        Ok(rule)
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
impl<'i> AstBuilder<'i> for UnionStatementNode<'i> {
    type Output = ();

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let name = self.name().build(ctx, state)?;
        let mut branches = IndexMap::default();
        for (index, branch) in self.union_block().union_branch().iter().enumerate() {
            let key = branch.branch_name(self.name().get_str(), index);

            if !branch.is_single() {
                let name = YggdrasilIdentifier { text: key.0.to_string(), span: FileID::default().with_range(key.1) };
                let term = branch.expression_hard().build(ctx, state)?;
                let rule =
                    GrammarRule { name, body: GrammarBody::Class { term }, range: self.get_range(), ..Default::default() };

                match state.grammar.rules.insert(rule.name.text.clone(), rule) {
                    Some(_) => {}
                    None => {}
                }
            };
            match branches.insert(key.0.to_string(), key.0.to_string()) {
                Some(old) => {
                    panic!("{old}")
                }
                None => {}
            }
        }
        let rule = GrammarRule {
            name,
            body: GrammarBody::Union { branches: vec![], refined: branches },
            range: self.get_range(),
            ..Default::default()
        }
        .with_annotation(self.annotations());
        state.grammar.rules.insert(rule.name.text.clone(), rule);
        Ok(())
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
                Ok(o) => out.push(o.with_annotation(self.annotations())),
                Err(_) => {}
            }
        }
        Ok((name, out))
    }
}

impl<'i> AstBuilder<'i> for ExpressionNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        match self.expression_hard().as_slice() {
            [head, rest @ ..] => {
                let mut head = head.build(ctx, state)?;
                for term in rest {
                    head |= term.build(ctx, state)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class or", self.get_range()))?,
        }
    }
}

impl<'i> AstBuilder<'i> for ExpressionHardNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        match self.expression_soft().as_slice() {
            [head, rest @ ..] => {
                let mut head = head.build(ctx, state)?;
                for term in rest {
                    head += term.build(ctx, state)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class hard", self.get_range()))?,
        }
    }
}

impl<'i> AstBuilder<'i> for ExpressionSoftNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        match self.expression_tag().as_slice() {
            [head, rest @ ..] => {
                let mut head = head.build(ctx, state)?;
                for term in rest {
                    head &= term.build(ctx, state)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class soft", self.get_range()))?,
        }
    }
}

impl<'i> AstBuilder<'i> for ExpressionTagNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let e = match &self.identifier() {
            Some(first) => self.term().build(ctx, state)?.with_tag(first.build(ctx, state)?),
            None => self.term().build(ctx, state)?,
        };
        Ok(e)
    }
}
impl<'i> AstBuilder<'i> for TermNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let mut base = self.atomic().build(ctx, state)?;
        let mut unary = Vec::with_capacity(self.prefix().len() + self.suffix().len());
        for i in self.suffix() {
            let suffix = match i {
                SuffixNode::Optional(_) => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::OPTIONAL),
                SuffixNode::Many(_) => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::MANY),
                SuffixNode::Many1(_) => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::MANY1),
                SuffixNode::RangeExact(u) => {
                    let i = u32::from_str_radix(u.integer().get_str(), 10).unwrap_or(u32::MAX);
                    YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(i, i))
                }
                SuffixNode::Range(v) => {
                    let min = match &v.min() {
                        Some(v) => u32::from_str_radix(v.get_str(), 10).unwrap_or(0),
                        None => 0,
                    };
                    let max = match &v.max() {
                        Some(v) => u32::from_str_radix(v.get_str(), 10).unwrap_or(u32::MAX),
                        None => u32::MAX,
                    };
                    YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(min, max))
                }
            };
            unary.push(suffix)
        }
        for i in self.prefix().iter().rev() {
            match i {
                PrefixNode::Negative(_) => unary.push(YggdrasilOperator::Negative),
                PrefixNode::Positive(_) => unary.push(YggdrasilOperator::Positive),
                PrefixNode::Remark(_) => base.remark = true,
            }
        }
        if unary.is_empty() { Ok(base) } else { Ok(UnaryExpression { base: Box::new(base), operators: unary }.into()) }
    }
}

impl<'i> AstBuilder<'i> for AtomicNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let expr = match self {
            AtomicNode::GroupExpression(e) => e.expression().build(ctx, state)?,
            AtomicNode::Boolean(v) => match v {
                BooleanNode::False(_) => YggdrasilExpression::boolean(false),
                BooleanNode::True(_) => YggdrasilExpression::boolean(true),
            },
            AtomicNode::FunctionCall(_) => {
                todo!()
            }
            AtomicNode::RegexEmbed(v) => YggdrasilRegex::new(v.get_str().trim(), v.get_range()).into(),
            AtomicNode::RegexRange(v) => YggdrasilRegex::new(v.get_str(), v.get_range()).into(),
            AtomicNode::StringRaw(s) => YggdrasilText::new(s.string_raw_text().get_str(), s.get_range()).into(),
            AtomicNode::StringNormal(s) => {
                let mut buffer = String::with_capacity(s.string_item().len() * 2);
                for item in &s.string_item() {
                    match item {
                        StringItemNode::EscapedCharacter(item) => match item.get_chars().last() {
                            Some(c) => match c {
                                'r' => buffer.push('\r'),
                                'n' => buffer.push('\n'),
                                _ => buffer.push(c),
                            },
                            None => unreachable!(),
                        },
                        StringItemNode::EscapedUnicode(unicode) => match unicode.as_char() {
                            Some(c) => buffer.push(c),
                            None => {
                                println!("parse fail: {:?}", unicode)
                            }
                        },
                        StringItemNode::TextAny(s) => buffer.push_str(s.get_str()),
                    }
                }
                YggdrasilText::new(buffer, s.get_range()).into()
            }
            AtomicNode::Category(cat) => {
                let r = cat.get_range();
                match &cat.group() {
                    Some(g) => YggdrasilRegex::new(format!("\\p{{{}={}}}", g.get_str(), cat.script().get_str()), r).into(),
                    None => YggdrasilRegex::new(format!("\\p{{{}}}", cat.script().get_str()), r).into(),
                }
            }
            AtomicNode::EscapedUnicode(unicode) => match unicode.as_char() {
                Some(c) => YggdrasilText::new(c, unicode.get_range()).into(),
                None => Err(YggdrasilError::syntax_error(unicode.hex().get_str(), unicode.get_range()))?,
            },
            AtomicNode::Identifier(v) => v.build(ctx, state)?.into(),
            AtomicNode::Integer(v) => BigInt::from_str_radix(v.get_str(), 10)?.into(),
        };
        Ok(expr)
    }
}

impl<'i> AstBuilder<'i> for IdentifierNode<'i> {
    type Output = YggdrasilIdentifier;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        Ok(YggdrasilIdentifier { text: self.get_str().to_string(), span: ctx.id.with_range(self.get_range()) })
    }
}

impl<'i> UnionBranchNode<'i> {
    pub fn is_single(&self) -> bool {
        self.expression_hard().to_identifier().is_some()
    }
    pub fn branch_name(&self, name: &str, index: usize) -> (Cow<str>, Range<usize>) {
        match self.try_branch_name() {
            Some(s) => s,
            None => match self.try_expression_name() {
                Some(s) => s,
                None => (Cow::Owned(format!("{}{}", name, index)), self.expression_hard().get_range()),
            },
        }
    }

    fn try_branch_name(&self) -> Option<(Cow<str>, Range<usize>)> {
        let branch = self.branch_tag()?;
        Some((Cow::Borrowed(branch.identifier().get_str()), branch.get_range()))
    }
    fn try_expression_name(&self) -> Option<(Cow<str>, Range<usize>)> {
        let atom = self.expression_hard().to_atom()?;
        let id = atom.to_identifier()?;
        Some((Cow::Borrowed(id.get_str()), id.get_range()))
    }
}
pub struct TakeAnnotations<'i> {
    auto_tag: bool,
    macros: Vec<DecoratorCallNode<'i>>,
    modifiers: Vec<ModifierCallNode<'i>>,
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
