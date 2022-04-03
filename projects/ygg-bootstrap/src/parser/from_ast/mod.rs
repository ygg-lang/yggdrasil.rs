use std::mem::take;

use crate::parser::ast::{CharsetNode, StringLiteral};
use peginator::PegParser;
use yggdrasil_error::{Diagnostic, YggdrasilError, YggdrasilResult};
use yggdrasil_ir::{
    ChoiceExpression, ExpressionKind, ExpressionNode, FunctionRule, GrammarInfo, GrammarRule, Operator, UnaryExpression,
};

use crate::parser::ast::{ChoiceNode, DefineStatement, Node, ProgramNode, ProgramParser, StatementNode, StringItem};

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
                body: ExpressionNode { kind: self.body.as_expr(ctx)?, branch_tag: "".to_string(), node_tag: "".to_string() },
                range: self.position.clone(),
            };
            ctx.info.rules.insert(rule.name.clone(), rule);
        }
        ctx.docs.clear();
        Ok(())
    }
}

impl ChoiceNode {
    fn as_expr(&self, ctx: &mut GrammarParser) -> Result<ExpressionKind, YggdrasilError> {
        let mut expr = ChoiceExpression::default();
        for term in &self.terms {
            let tag = term.tag.as_ref().map(|f| f.string.to_owned()).unwrap_or_default();
            let body = match &term.node {
                Node::Identifier(node) => ExpressionKind::rule(&node.string),
                Node::StringLiteral(node) => node.as_expr(ctx)?,
                Node::CharsetNode(node) => node.as_expr(ctx)?,
                Node::Group(node) => node.body.as_expr(ctx)?,
            };
            let mut ops = vec![];
            for suffix in &term.suffix {
                match suffix {
                    '?' => ops.push(Operator::Optional),
                    '*' => ops.push(Operator::Repeats),
                    '+' => ops.push(Operator::Repeat1),
                    _ => unreachable!(),
                }
            }
            for suffix in term.prefix.iter().rev() {
                match suffix {
                    '^' => ops.push(Operator::Remark),
                    '!' => ops.push(Operator::Negative),
                    _ => unreachable!(),
                }
            }
            if ops.is_empty() {
                expr.push(ExpressionNode { kind: body, branch_tag: "".to_string(), node_tag: tag })
            }
            else {
                let unary =
                    UnaryExpression { base: ExpressionNode { kind: body, branch_tag: "".to_string(), node_tag: tag }, ops };
                expr.push(ExpressionNode {
                    kind: ExpressionKind::Unary(Box::new(unary)),
                    branch_tag: "".to_string(),
                    node_tag: "".to_string(),
                })
            }
        }
        return Ok(ExpressionKind::Choice(Box::new(expr)));
    }
}
