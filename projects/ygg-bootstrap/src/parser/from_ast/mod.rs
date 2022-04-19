use crate::parser::ast::{CharsetNode, ExprStream, Infix, StringLiteral};
use peginator::PegParser;
use std::mem::take;
use yggdrasil_error::{Diagnostic, YggdrasilError, YggdrasilResult};
use yggdrasil_ir::{
    ChoiceExpression, ConcatExpression, ExpressionKind, ExpressionNode, FunctionRule, GrammarInfo, GrammarRule, Operator,
};
use yggdrasil_rt::traits::{Affix, PrattParser};

use crate::parser::ast::{ChoiceNode, DefineStatement, ProgramNode, ProgramParser, StatementNode, StringItem};

mod charset;
mod import;

pub struct GrammarParser {
    info: GrammarInfo,
    docs: String,
    errors: Vec<YggdrasilError>,
}

impl GrammarParser {
    pub fn parse(input: &str) -> YggdrasilResult<GrammarInfo> {
        let mut ctx = GrammarParser { info: Default::default(), docs: "".to_string(), errors: vec![] };
        ProgramParser::parse(input).unwrap().program.translate(&mut ctx)?;
        Ok(Diagnostic { success: ctx.info, errors: ctx.errors })
    }
}

impl ProgramNode {
    fn translate(self, ctx: &mut GrammarParser) -> YggdrasilResult {
        for s in self.statements {
            match s {
                StatementNode::DefineStatement(define) => define.translate(ctx)?,
                StatementNode::EmptyStatement(_) => {}
            }
        }
        Ok(Diagnostic { success: (), errors: vec![] })
    }
}

impl DefineStatement {
    pub fn annotation(&self, name: &str, default: bool) -> bool {
        let mut field = default;
        if self.modifiers.id.iter().filter(|f| f.string == name).next().is_some() {
            field = true
        }
        return field;
    }

    fn translate(&self, ctx: &mut GrammarParser) -> Result<(), YggdrasilError> {
        let document = take(&mut ctx.docs);
        let mut name = self.symbol.string.to_owned();
        let mut auto_inline = self.annotation("inline", false);
        if name.starts_with('_') {
            auto_inline = true;
            name = name.trim_start_matches("_").to_string()
        }
        let mut r#type = String::new();
        if let Some(s) = &self.r#type {
            r#type = s.id.string.to_owned()
        }
        if self.arguments.is_some() {
            let _ = FunctionRule {};
        }
        else {
            let rule = GrammarRule {
                name,
                r#type,
                document,
                derives: Default::default(),
                auto_inline,
                auto_boxed: self.annotation("boxed", false),
                auto_capture: self.annotation("capture", true),
                atomic: self.annotation("atomic", false),
                entry: self.annotation("entry", false),
                keep: self.annotation("keep", false),
                body: self.body.as_expr(ctx)?,
                range: self.position.clone(),
            };
            ctx.info.rules.insert(rule.name.clone(), rule);
        }
        ctx.docs.clear();
        Ok(())
    }
}

struct ExprParser<'ctx> {
    ctx: &'ctx mut GrammarParser,
}

impl ExprStream {
    pub fn as_infix(&self) -> Option<&str> {
        match self {
            ExprStream::Infix(i) => Some(i.string.as_str()),
            _ => None,
        }
    }
    pub fn as_prefix(&self) -> Option<&str> {
        match self {
            ExprStream::Prefix(i) => Some(i.string.as_str()),
            _ => None,
        }
    }
    pub fn as_suffix(&self) -> Option<&str> {
        match self {
            ExprStream::Suffix(i) => Some(i.string.as_str()),
            _ => None,
        }
    }
}

// noinspection DuplicatedCode
impl<'i> PrattParser for ExprParser<'i> {
    type Input = ExprStream;
    type Output = ExpressionNode;

    fn query(&mut self, tree: &ExprStream) -> Result<Affix, YggdrasilError> {
        let affix = match tree {
            ExprStream::Prefix(prefix) => match prefix.string.as_str() {
                "^" | "!" => Affix::prefix(10),
                _ => unreachable!("{}", prefix.string.as_str()),
            },
            ExprStream::Infix(infix) => match infix.string.as_str() {
                "|" => Affix::infix_left(1),
                "~" | "&" => Affix::infix_left(2),
                ":" => Affix::infix_left(3),
                _ => unreachable!("{}", infix.string.as_str()),
            },
            ExprStream::Suffix(suffix) => match suffix.string.as_str() {
                "?" | "+" | "*" => Affix::suffix(20),
                _ => unreachable!("{}", suffix.string.as_str()),
            },
            _ => Affix::None,
        };
        Ok(affix)
    }

    fn primary(&mut self, tree: ExprStream) -> Result<ExpressionNode, YggdrasilError> {
        let expr = match tree {
            ExprStream::CharsetNode(_v) => {
                todo!()
            }
            ExprStream::Group(v) => return v.body.as_expr(self.ctx),
            ExprStream::Identifier(v) => ExpressionKind::rule(&v.string),
            ExprStream::Infix(_) => {
                unreachable!()
            }
            ExprStream::Prefix(_) => {
                unreachable!()
            }
            ExprStream::Suffix(_) => {
                unreachable!()
            }
            ExprStream::StringLiteral(v) => v.as_expr(self.ctx)?,
        };
        Ok(ExpressionNode { kind: expr, branch_tag: "".to_string(), node_tag: "".to_string() })
    }

    fn infix(&mut self, lhs: ExpressionNode, tree: ExprStream, rhs: ExpressionNode) -> Result<ExpressionNode, YggdrasilError> {
        let kind = match tree.as_infix() {
            Some("~") => ExpressionKind::Concat(box ConcatExpression::new(lhs, rhs, true)),
            Some("&") => ExpressionKind::Concat(box ConcatExpression::new(lhs, rhs, false)),
            Some("|") => return Ok(lhs | rhs),
            Some(":") => return lhs / rhs,
            _ => unreachable!(),
        };
        Ok(ExpressionNode { kind, branch_tag: "".to_string(), node_tag: "".to_string() })
    }

    fn prefix(&mut self, tree: Self::Input, rhs: Self::Output) -> Result<Self::Output, YggdrasilError> {
        let op = match tree.as_prefix() {
            Some("^") => Operator::Remark,
            Some("!") => Operator::Negative,
            _ => unreachable!(),
        };
        Ok(rhs + op)
    }

    fn suffix(&mut self, lhs: Self::Output, tree: Self::Input) -> Result<Self::Output, YggdrasilError> {
        let op = match tree.as_suffix() {
            Some("?") => Operator::Optional,
            Some("*") => Operator::Repeats,
            Some("+") => Operator::Repeat1,
            _ => unreachable!(),
        };
        Ok(lhs + op)
    }
}

impl ChoiceNode {
    fn as_expr(&self, ctx: &mut GrammarParser) -> Result<ExpressionNode, YggdrasilError> {
        if self.terms.is_empty() {
            Ok(ExpressionNode::empty())
        }
        else {
            let mut pratt = ExprParser { ctx };
            let mut normed = vec![];
            let mut last = false;
            for term in &self.terms {
                let this = match term {
                    ExprStream::Prefix(_) => false,
                    ExprStream::Infix(_) => false,
                    ExprStream::Suffix(_) => false,
                    ExprStream::CharsetNode(_) => true,
                    ExprStream::Group(_) => true,
                    ExprStream::Identifier(_) => true,
                    ExprStream::StringLiteral(_) => true,
                };
                if last && this {
                    normed.push(ExprStream::Infix(Infix { string: "&".to_string(), position: Default::default() }))
                }
                normed.push(term.clone());
                last = this;
            }
            pratt.parse(&normed)
        }
    }
}
