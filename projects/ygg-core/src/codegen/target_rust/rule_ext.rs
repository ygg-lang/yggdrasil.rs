use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::{fmt::Debug, ops::Range};
use yggdrasil_ir::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ExpressionKind, ExpressionNode, Operator},
    rule::{GrammarRule, GrammarRuleKind, RuleDerive, YggdrasilIdentifier},
    traits::FieldMap,
};

use super::*;

pub(super) trait RuleExt {
    fn safe_rule_name(&self) -> String;
    fn parser_name(&self) -> String;

    fn parser_expression(&self) -> String;
}

impl RuleExt for GrammarRule {
    fn safe_rule_name(&self) -> String {
        self.name.text.to_string()
    }

    fn parser_name(&self) -> String {
        format!("parse_{}", self.name.text).to_case(Case::Snake)
    }

    fn parser_expression(&self) -> String {
        let mut w = String::new();
        match &self.body {
            Some(s) => match s.write(&mut w, self) {
                Ok(_) => {}
                Err(_) => w.push_str("parse_Bug(s)"),
            },
            None => w.push_str("parse_Bug(s)"),
        }
        w
    }
}

trait NodeExt {
    fn write(&self, w: &mut String, ctx: &GrammarRule) -> std::fmt::Result;
}

impl NodeExt for ExpressionNode {
    fn write(&self, w: &mut String, ctx: &GrammarRule) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Function(_) => w.push_str("parse_Function(s)"),
            ExpressionKind::Choice(v) => {
                for branch in &v.branches {
                    writeln!(w, "// {:?}", branch)?
                }
            }
            ExpressionKind::Concat(v) => {
                let (head, rest) = v.split();
                w.push_str("s.sequence(|s|");
                head.write(w, ctx)?;
                for pat in rest {
                    w.push_str(".and_then(|s|");
                    pat.write(w, ctx)?;
                    w.push_str(")");
                }
                w.push_str(")")
            }
            ExpressionKind::Unary(v) => {
                for o in &v.operators {
                    match o {
                        Operator::Negative => {}
                        Operator::Optional => {
                            w.push_str("s.optional(|s|");
                        }
                        Operator::Repeats => {}
                        Operator::Repeat1 => {}
                        Operator::Boxing => {}
                        Operator::RepeatsBetween(_, _) => {}
                        Operator::Remark => {}
                        Operator::Recursive => {}
                    }
                }
                v.base.write(w, ctx)?;
                for _ in &v.operators {
                    w.push_str(")")
                }
            }
            ExpressionKind::Rule(r) => {
                let name = format!("parse_{}", r.name.text).to_case(Case::Snake);
                write!(w, "{name}(s)")?
            }
            ExpressionKind::Text(v) => write!(w, "s.match_string({:?})", v.text)?,
            ExpressionKind::Regex(_) => w.push_str("parse_Regex(s)"),
            ExpressionKind::Data(_) => w.push_str("parse_Unary(s)"),
        }
        Ok(())
    }
}

pub struct ClassObject {
    name: String,
    derives: RuleDerive,
    file: FieldMap,
}
