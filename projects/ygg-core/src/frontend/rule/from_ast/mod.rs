use peginator::PegParser;
use std::{
    collections::{BTreeSet, HashSet},
    mem::take,
    str::FromStr,
};

use url::Url;

use yggdrasil_bootstrap::{
    parser::{Choice, DefineStatement, Node, Program, Statement, StringItem, StringLiteral, Typing},
    Result,
};

use crate::frontend::{
    rule::{node::ExpressionKind, ChoiceExpression, DataKind, ExpressionNode},
    GrammarInfo, GrammarRule, Symbol,
};
use crate::frontend::rule::UnaryExpression;

mod import;
mod macros;

impl GrammarInfo {
    pub fn parse(input: &str) -> Result<Self> {
        let mut ctx = GrammarContext { info: Default::default(), docs: "".to_string() };
        let pro = Program::parse(input).unwrap();
        pro.translate(&mut ctx)?;
        Ok(ctx.info)
    }
}

pub struct GrammarContext {
    pub(crate) info: GrammarInfo,
    docs: String,
}

trait Translator
where
    Self: Sized,
{
    fn translate(self, ctx: &mut GrammarContext) -> Result<()> {
        let _ = ctx;
        unimplemented!()
    }
    fn into_expr(self, ctx: &mut GrammarContext) -> Result<ExpressionNode> {
        let _ = ctx;
        unimplemented!()
    }
    fn into_data(self, ctx: &mut GrammarContext) -> Result<DataKind> {
        let _ = ctx;
        unimplemented!()
    }
}

impl Translator for Program {
    fn translate(self, ctx: &mut GrammarContext) -> Result<()> {
        for s in self.statements {
            match s {
                Statement::DefineStatement(define) => define.translate(ctx)?,
                Statement::EmptyStatement(_) => {}
            }
        }
        Ok(())
    }
}

impl Translator for DefineStatement {
    fn translate(self, ctx: &mut GrammarContext) -> Result<()> {
        let document = take(&mut ctx.docs);
        let mut name = self.symbol.string.to_owned();
        let mut modifiers: HashSet<String> = Default::default();
        for id in self.modifiers.id {
            modifiers.insert(id.string);
        }
        let mut auto_inline = false;
        if modifiers.contains("inline") {
            auto_inline = true
        }
        else if name.starts_with('_') {
            auto_inline = true;
            name = name.trim_start_matches("_").to_string()
        }
        let mut auto_boxed = false;
        if modifiers.contains("boxed") {
            auto_boxed = true
        }
        let mut atomic_rule = false;
        if modifiers.contains("atomic") {
            atomic_rule = true
        }
        let mut auto_capture = true;

        let mut r#type = String::new();
        if let Some(s) = self.r#type {
            r#type = s.id.string
        }
        if self.arguments.is_some() {
        }
        else {
            let rule = GrammarRule {
                name,
                r#type,
                document,
                derives: Default::default(),
                auto_inline,
                auto_boxed,
                auto_capture,
                atomic_rule,
                body: self.body.into_expr(ctx)?,
                range: self.position,
            };
            ctx.info.rules.insert(rule.name.clone(), rule);
        }
        Ok(ctx.docs.clear())
    }
}

impl Translator for Choice {
    fn into_expr(self, ctx: &mut GrammarContext) -> Result<ExpressionNode> {
        let mut expr = ChoiceExpression::default();
        for term in self.terms {
            let tag = term.tag.map(|f| f.string).unwrap_or_default();
            let body = match term.node {
                Node::Identifier(node) => ExpressionNode { tag, kind: ExpressionKind::rule(&node.string) },
                Node::StringLiteral(node) => {
                    let mut s = String::new();
                    for item in node.body {
                        match item {
                            StringItem::CharOne(c) => s.push(c),
                            StringItem::StringEscaped(escaped) => match escaped.char {
                                'n' => s.push('\n'),
                                _ => s.push(escaped.char),
                            },
                        }
                    }
                    ExpressionNode { tag, kind: ExpressionKind::string(s) }
                },
                Node::Charset(node) => {
                    unimplemented!()
                }
                Node::Group(node) => {
                    unimplemented!()
                }
            };
            let mut ops = vec![];
            for suffix in term.suffix {

            }
            if ops.is_empty() {

            }
            else {
                ExpressionNode {

                }

                ExpressionKind::Unary(UnaryExpression {
                    tag,
                    base: (),
                    ops: vec![]
                })

            }
        }
        return Ok(ExpressionNode { tag: "".to_string(), kind: ExpressionKind::Choice(Box::new(expr)) });
    }
}
