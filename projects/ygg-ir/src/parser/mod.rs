use indexmap::IndexMap;
use num::{BigInt, Num};
use std::collections::BTreeMap;

use yggdrasil_error::{Failure, FileCache, FileID, FileSpan, Success, Validation, YggdrasilError};
use yggdrasil_parser::{
    bootstrap::{
        AtomicNode, BooleanNode, ClassStatementNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode,
        GrammarStatementNode, GroupPairNode, GroupStatementNode, IdentifierNode, PrefixNode, RootNode, StatementNode, StringItemNode,
        SuffixNode, TermNode, UnionBranchNode, UnionStatementNode,
    },
    TakeAnnotations, YggdrasilNode,
};

use crate::{
    data::{YggdrasilRegex, YggdrasilText},
    grammar::GrammarInfo,
    nodes::{UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{GrammarAtomic, GrammarBody, GrammarRule, YggdrasilCounter, YggdrasilIdentifier, YggdrasilVariant},
};

mod annotations;
use crate::utils::ParseContext;
use yggdrasil_error::Result;

impl GrammarInfo {
    pub fn new(id: FileID, cache: &mut FileCache) -> Validation<Self> {
        let text = match cache.fetch(&id) {
            Ok(o) => o.to_string(),
            Err(e) => Err(YggdrasilError::from(e))?,
        };
        let root = match RootNode::from_str(&text) {
            Ok(o) => o,
            Err(e) => Err(YggdrasilError::from(e).with_file(id))?,
        };
        let mut ctx = ParseContext::new(id);
        match Self::build(root, &mut ctx) {
            Ok(value) => Success { value, diagnostics: ctx.get_errors() },
            Err(fatal) => Failure { fatal, diagnostics: ctx.get_errors() },
        }
    }
}

impl GrammarInfo {
    pub(crate) fn build(root: RootNode, ctx: &mut ParseContext) -> Result<Self> {
        let mut out = GrammarInfo::default();
        for s in &root.statement {
            match s {
                StatementNode::GrammarStatement(v) => out.visit_grammar(v)?,
                StatementNode::ClassStatement(v) => match GrammarRule::build_class(v) {
                    Ok(o) => {
                        out.rules.insert(o.name.text.clone(), o);
                    }
                    Err(e) => ctx.add_error(e),
                },
                StatementNode::UnionStatement(v) => GrammarRule::register_union(v, &mut out)?,
                StatementNode::GroupStatement(v) => match GrammarRule::build_group(v) {
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
                    Err(e) => ctx.add_error(e),
                },
            }
        }
        Ok(out)
    }
}

impl GrammarInfo {
    fn visit_grammar(&mut self, node: &GrammarStatementNode) -> Result<()> {
        self.name = YggdrasilIdentifier::build(&node.identifier);
        Ok(())
    }
}

impl GrammarRule {
    fn build_class(node: &ClassStatementNode) -> Result<Self> {
        let name = YggdrasilIdentifier::build(&node.name);
        let rule = Self {
            name,
            body: GrammarBody::Class { term: YggdrasilExpression::build_or(&node.class_block.expression)? },
            range: node.get_range(),
            ..Default::default()
        }
        .with_annotation(node.annotations());
        Ok(rule)
    }
    fn build_class_in_group(node: &GroupPairNode) -> Result<Self> {
        let name = YggdrasilIdentifier::build(&node.identifier);
        let rule = Self {
            name,
            body: GrammarBody::Class { term: YggdrasilExpression::build_atomic(&node.atomic)? },
            range: node.get_range(),
            ..Default::default()
        };
        Ok(rule)
    }
    fn register_union(node: &UnionStatementNode, rules: &mut GrammarInfo) -> Result<()> {
        let name = YggdrasilIdentifier::build(&node.name);
        let mut branches = IndexMap::default();
        for (index, branch) in node.union_block.union_branch.iter().enumerate() {
            let key = branch.branch_name(&node.name.text, index);

            if !branch.is_single() {
                let name = YggdrasilIdentifier { text: key.0.to_string(), span: FileID::default().with_range(key.1) };
                let term = YggdrasilExpression::build_hard(&branch.expression_hard)?;
                let rule = GrammarRule { name, body: GrammarBody::Class { term }, range: node.get_range(), ..Default::default() };

                match rules.rules.insert(rule.name.text.clone(), rule) {
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
        let rule = Self { name, body: GrammarBody::Union { branches }, range: node.get_range(), ..Default::default() }
            .with_annotation(node.annotations());
        rules.rules.insert(rule.name.text.clone(), rule);
        Ok(())
    }
    fn build_group(node: &GroupStatementNode) -> Result<(Option<YggdrasilIdentifier>, Vec<Self>)> {
        let name = node.identifier.as_ref().map(YggdrasilIdentifier::build);
        let mut out = vec![];
        for term in &node.group_block.group_pair {
            match GrammarRule::build_class_in_group(term) {
                Ok(o) => out.push(o.with_annotation(node.annotations())),
                Err(_) => {}
            }
        }
        Ok((name, out))
    }
}

impl YggdrasilExpression {
    fn build_or(node: &ExpressionNode) -> Result<Self> {
        match node.expression_hard.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_hard(head)?;
                for term in rest {
                    head |= YggdrasilExpression::build_hard(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class or", node.get_range()))?,
        }
    }

    fn build_hard(node: &ExpressionHardNode) -> Result<Self> {
        match node.expression_soft.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_soft(head)?;
                for term in rest {
                    head += YggdrasilExpression::build_soft(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class hard", node.get_range()))?,
        }
    }
    fn build_soft(node: &ExpressionSoftNode) -> Result<Self> {
        match node.expression_tag.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_tag_node(head)?;
                for term in rest {
                    head &= YggdrasilExpression::build_tag_node(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class soft", node.get_range()))?,
        }
    }
    fn build_tag_node(node: &ExpressionTagNode) -> Result<Self> {
        let e = match &node.identifier {
            Some(first) => YggdrasilExpression::build_term(&node.term)?.with_tag(YggdrasilIdentifier::build(first)),
            None => YggdrasilExpression::build_term(&node.term)?,
        };
        Ok(e)
    }
    fn build_term(node: &TermNode) -> Result<Self> {
        let mut base = YggdrasilExpression::build_atomic(&node.atomic)?;
        let mut unary = Vec::with_capacity(node.prefix.len() + node.suffix.len());
        for i in &node.suffix {
            let suffix = match i {
                SuffixNode::Optional => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::OPTIONAL),
                SuffixNode::Many => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::MANY),
                SuffixNode::Many1 => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::MANY1),
                SuffixNode::RangeExact(u) => {
                    let i = u32::from_str_radix(&u.integer.text, 10).unwrap_or(u32::MAX);
                    YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(i, i))
                }
                SuffixNode::Range(v) => {
                    let min = match &v.min {
                        Some(v) => u32::from_str_radix(&v.text, 10).unwrap_or(0),
                        None => 0,
                    };
                    let max = match &v.max {
                        Some(v) => u32::from_str_radix(&v.text, 10).unwrap_or(u32::MAX),
                        None => u32::MAX,
                    };
                    YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(min, max))
                }
            };
            unary.push(suffix)
        }
        for i in node.prefix.iter().rev() {
            match i {
                PrefixNode::Negative => unary.push(YggdrasilOperator::Negative),
                PrefixNode::Positive => unary.push(YggdrasilOperator::Positive),
                PrefixNode::Remark => base.remark = true,
            }
        }
        if unary.is_empty() { Ok(base) } else { Ok(UnaryExpression { base: Box::new(base), operators: unary }.into()) }
    }
    fn build_atomic(node: &AtomicNode) -> Result<Self> {
        let expr = match node {
            AtomicNode::GroupExpression(e) => YggdrasilExpression::build_or(&e.expression)?,
            AtomicNode::Boolean(v) => match v {
                BooleanNode::False => YggdrasilExpression::boolean(false),
                BooleanNode::True => YggdrasilExpression::boolean(true),
            },
            AtomicNode::FunctionCall(_) => {
                todo!()
            }
            AtomicNode::RegexEmbed(v) => YggdrasilRegex::new(v.to_string().trim(), v.span.clone()).into(),
            AtomicNode::RegexRange(v) => YggdrasilRegex::new(&v.text, v.get_range()).into(),
            AtomicNode::StringRaw(s) => YggdrasilText::new(&s.string_raw_text.text, s.get_range()).into(),
            AtomicNode::StringNormal(s) => {
                let mut buffer = String::with_capacity(s.string_item.len() * 2);
                for item in &s.string_item {
                    match item {
                        StringItemNode::EscapedCharacter(item) => match item.text.chars().last() {
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
                        StringItemNode::TextAny(s) => buffer.push_str(&s.text),
                    }
                }
                YggdrasilText::new(buffer, s.get_range()).into()
            }
            AtomicNode::Category(cat) => {
                let r = cat.get_range();
                match &cat.group {
                    Some(g) => YggdrasilRegex::new(format!("\\p{{{}={}}}", g.text, cat.script.text), r).into(),
                    None => YggdrasilRegex::new(format!("\\p{{{}}}", cat.script.text), r).into(),
                }
            }
            AtomicNode::EscapedUnicode(unicode) => match unicode.as_char() {
                Some(c) => YggdrasilText::new(c, unicode.get_range()).into(),
                None => Err(YggdrasilError::syntax_error(&unicode.hex.text, unicode.get_range()))?,
            },
            AtomicNode::Identifier(v) => YggdrasilIdentifier::build(v).into(),
            AtomicNode::Integer(v) => BigInt::from_str_radix(&v.text, 10)?.into(),
        };
        Ok(expr)
    }
}

impl YggdrasilIdentifier {
    fn build(node: &IdentifierNode) -> Self {
        Self { text: node.text.clone(), span: unsafe { FileID::new(0) }.with_range(node.get_range()) }
    }
}
