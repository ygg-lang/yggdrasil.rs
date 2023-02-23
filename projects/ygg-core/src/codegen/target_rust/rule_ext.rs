use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::{fmt::Debug, ops::Range};
use yggdrasil_ir::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ExpressionKind, Operator, YggdrasilExpression},
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
        let raw = self.name.text.as_str();
        let keywords = &[
            "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else", "enum",
            "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut",
            "offsetof", "override", "priv", "proc", "pub", "pure", "ref", "return", "Self", "self", "sizeof", "static",
            "struct", "super", "trait", "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while",
            "yield",
        ];
        if keywords.contains(&raw) { format!("r#{raw}") } else { raw.to_string() }
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

impl NodeExt for YggdrasilExpression {
    fn write(&self, w: &mut String, ctx: &GrammarRule) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Ignored => w.push_str("builtin_ignore(s)"),
            ExpressionKind::Function(_) => w.push_str("parse_Function(s)"),
            ExpressionKind::Choice(v) => {
                let (head, rest) = v.split();
                head.write(w, ctx)?;
                for pat in rest {
                    w.push_str(".or_else(|s|");
                    pat.write(w, ctx)?;
                    w.push_str(")");
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
                        Operator::Negative => {
                            todo!()
                        }
                        Operator::Optional => {
                            w.push_str("s.optional(|s|");
                        }
                        Operator::Repeats => {
                            todo!()
                        }
                        Operator::Repeat1 => {
                            todo!()
                        }
                        Operator::Boxing => {
                            todo!()
                        }
                        Operator::RepeatsBetween(_, _) => {
                            todo!()
                        }
                        Operator::Remark => {
                            todo!()
                        }
                        Operator::Recursive => {
                            todo!()
                        }
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
            ExpressionKind::Text(v) if v.insensitive => write!(w, "builtin_text::<true>(s, {:?})", v.text)?,
            ExpressionKind::Text(v) => write!(w, "builtin_text::<false>(s, {:?})", v.text)?,
            ExpressionKind::Regex(r) => w.push_str("parse_Regex(s)"),
            ExpressionKind::Data(_) => w.push_str("parse_Data(s)"),
            ExpressionKind::CharacterAny => w.push_str("s.match_char_by(|_| true)"),
            ExpressionKind::Boolean(_) => {}
        }
        Ok(())
    }
}

pub struct ClassObject {
    name: String,
    derives: RuleDerive,
    file: FieldMap,
}
