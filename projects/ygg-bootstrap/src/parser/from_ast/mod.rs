use crate::parser::ast::{CharsetNode, ExprStream, Prefix, StringLiteral};
use peginator::PegParser;
use pratt::{Affix, Associativity, NoError, PrattError, PrattParser, Precedence};
use std::mem::take;
use yggdrasil_error::{Diagnostic, YggdrasilError, YggdrasilResult};
use yggdrasil_ir::{
    ChoiceExpression, ConcatExpression, ExpressionKind, ExpressionNode, FunctionRule, GrammarInfo, GrammarRule, Operator,
    UnaryExpression,
};

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
                body: ExpressionNode { kind: self.body.as_expr(ctx)?, branch_tag: "".to_string(), node_tag: "".to_string() },
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

impl<'i, I> PrattParser<I> for ExprParser<'i>
where
    I: Iterator<Item = ExprStream>,
{
    type Error = YggdrasilError;
    type Input = ExprStream;
    type Output = ExpressionNode;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &ExprStream) -> Result<Affix, YggdrasilError> {
        let affix = match tree {
            ExprStream::Infix('=') => Affix::Infix(Precedence(2), Associativity::Neither),
            ExprStream::Infix('+') => Affix::Infix(Precedence(3), Associativity::Left),
            ExprStream::Infix('-') => Affix::Infix(Precedence(3), Associativity::Left),
            ExprStream::Infix('*') => Affix::Infix(Precedence(4), Associativity::Left),
            ExprStream::Infix('/') => Affix::Infix(Precedence(4), Associativity::Left),
            ExprStream::Suffix('?') => Affix::Postfix(Precedence(5)),
            ExprStream::Prefix('-') => Affix::Prefix(Precedence(6)),
            ExprStream::Prefix('!') => Affix::Prefix(Precedence(6)),
            ExprStream::Infix('^') => Affix::Infix(Precedence(7), Associativity::Right),
            ExprStream::Group(_) | ExprStream::CharsetNode(_) | ExprStream::Identifier(_) | ExprStream::StringLiteral(_) => {
                Affix::Nilfix
            }
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: ExprStream) -> Result<ExpressionNode, YggdrasilError> {
        let expr = match tree {
            ExprStream::CharsetNode(v) => {
                todo!()
            }
            ExprStream::Group(v) => v.body.as_expr(self.ctx)?,
            ExprStream::Identifier(v) => {
                todo!()
            }
            ExprStream::Infix(_) => {
                unreachable!()
            }
            ExprStream::Prefix(_) => {
                unreachable!()
            }
            ExprStream::Suffix(_) => {
                unreachable!()
            }
            ExprStream::StringLiteral(v) => {
                todo!()
            }
        };
        Ok(ExpressionNode { kind: expr, branch_tag: "".to_string(), node_tag: "".to_string() })
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: ExpressionNode, tree: ExprStream, rhs: ExpressionNode) -> Result<ExpressionNode, YggdrasilError> {
        let kind = match tree {
            ExprStream::Infix('~') => ExpressionKind::Concat(box ConcatExpression::new(lhs, rhs, true)),
            ExprStream::Infix('|') => ExpressionKind::Choice(box ChoiceExpression::new(lhs, rhs)),
            ExprStream::Infix(':') => match lhs.kind.as_tag() {
                Some(s) => return Ok(ExpressionNode { kind: rhs.kind, branch_tag: rhs.branch_tag, node_tag: s.to_string() }),
                None => {
                    unreachable!()
                }
            },
            _ => unreachable!(),
        };
        Ok(ExpressionNode { kind, branch_tag: "".to_string(), node_tag: "".to_string() })
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: ExprStream, rhs: ExpressionNode) -> Result<ExpressionNode, YggdrasilError> {
        let op = match tree {
            ExprStream::Prefix(prefix) => {
                let op = match prefix {
                    '^' => Operator::Remark,
                    _ => unreachable!(),
                };
                UnaryExpression { base: rhs, ops: vec![op] }
            }
            _ => unreachable!(),
        };
        Ok(ExpressionNode { kind: ExpressionKind::Unary(box op), branch_tag: "".to_string(), node_tag: "".to_string() })
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ExpressionNode, tree: ExprStream) -> Result<ExpressionNode, YggdrasilError> {
        let op = match tree {
            ExprStream::Suffix(suffix) => {
                let op = match suffix {
                    '?' => Operator::Optional,
                    '*' => Operator::Repeats,
                    '+' => Operator::Repeat1,
                    _ => unreachable!(),
                };
                UnaryExpression { base: lhs, ops: vec![op] }
            }
            _ => unreachable!(),
        };
        Ok(ExpressionNode { kind: ExpressionKind::Unary(box op), branch_tag: "".to_string(), node_tag: "".to_string() })
    }
}

impl ChoiceNode {
    fn as_expr(&self, ctx: &mut GrammarParser) -> Result<ExpressionKind, YggdrasilError> {
        let expr = ExprParser { ctx }.parse(self.terms.iter());
        match expr {
            Ok(o) => Ok(o),
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
}
