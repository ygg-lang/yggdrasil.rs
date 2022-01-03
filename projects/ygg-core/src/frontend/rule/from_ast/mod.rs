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
    rule::{node::Expression, ChoiceExpression, DataExpression, DataKind},
    GrammarInfo, GrammarRule, Symbol,
};

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
    fn into_expr(self, ctx: &mut GrammarContext) -> Result<Expression> {
        let _ = ctx;
        unimplemented!()
    }
    fn into_data(self, ctx: &mut GrammarContext) -> Result<DataExpression> {
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
            println!("{:#?}", self.body);
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
    fn into_expr(self, ctx: &mut GrammarContext) -> Result<Expression> {
        let mut expr = ChoiceExpression::default();
        for term in self.terms {
            match term.node {
                Node::Charset(node) => {
                    unimplemented!()
                }
                Node::Group(node) => {
                    unimplemented!()
                }
                Node::StringLiteral(node) => expr.push(Expression::Data(Box::new(node.into_data(ctx)?))),
                Node::Identifier(node) => {
                    let kind = DataKind::Rule(node.string.starts_with('_'), node.string.trim_start_matches("_").to_string());
                    expr.push(Expression::Data(Box::new(DataExpression { tag: "".to_string(), kind })))
                }
            }
        }

        return Ok(Expression::Choice(Box::new(expr)));
    }
}

impl Translator for StringLiteral {
    fn into_data(self, ctx: &mut GrammarContext) -> Result<DataExpression> {
        let mut s = String::new();
        for item in self.body {
            match item {
                StringItem::CharOne(c) => s.push(c),
                StringItem::StringEscaped(escaped) => match escaped.char {
                    'n' => s.push('\n'),
                    _ => s.push(escaped.char),
                },
            }
        }
        Ok(DataExpression { tag: "".to_string(), kind: DataKind::AnyCharacter })
    }
}
