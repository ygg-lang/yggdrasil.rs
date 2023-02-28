use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::{fmt::Debug, ops::Range};
use yggdrasil_ir::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ExpressionKind, YggdrasilExpression, YggdrasilOperator},
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
            Some(s) => match s.write(&mut w, self, true) {
                Ok(_) => {}
                Err(_) => w.push_str("parse_Bug(s)"),
            },
            None => w.push_str("parse_Bug(s)"),
        }
        w
    }
}

trait NodeExt {
    fn write(&self, w: &mut String, ctx: &GrammarRule, root: bool) -> std::fmt::Result;
}

impl NodeExt for YggdrasilExpression {
    fn write(&self, w: &mut String, ctx: &GrammarRule, root: bool) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Ignored => w.push_str("builtin_ignore(s)"),
            ExpressionKind::Function(_) => w.push_str("parse_Function(s)"),
            ExpressionKind::Choice(v) => {
                let (head, rest) = v.split();
                head.write(w, ctx, false)?;
                for pat in rest {
                    w.push_str(".or_else(|s|");
                    pat.write(w, ctx, false)?;
                    w.push_str(")");
                }
            }
            ExpressionKind::Concat(v) => {
                let (head, rest) = v.split();
                w.push_str("s.sequence(|s|");
                head.write(w, ctx, false)?;
                for pat in rest {
                    w.push_str(".and_then(|s|");
                    pat.write(w, ctx, false)?;
                    w.push_str(")");
                }
                w.push_str(")")
            }
            ExpressionKind::Unary(v) => {
                for o in &v.operators {
                    match o {
                        YggdrasilOperator::Negative => {
                            todo!()
                        }
                        YggdrasilOperator::Optional => {
                            w.push_str("s.optional(|s|");
                        }
                        YggdrasilOperator::Repeats => {
                            todo!()
                        }
                        YggdrasilOperator::Repeat1 => {
                            todo!()
                        }
                        YggdrasilOperator::Boxing => {
                            todo!()
                        }
                        YggdrasilOperator::RepeatsBetween(_, _) => {
                            todo!()
                        }
                        YggdrasilOperator::Recursive => {
                            todo!()
                        }
                    }
                }
                v.base.write(w, ctx, false)?;
                for _ in &v.operators {
                    w.push_str(")")
                }
            }
            ExpressionKind::Rule(r) => {
                let name = format!("parse_{}", r.name.text).to_case(Case::Snake);
                write!(w, "{name}(s)")?
            }
            ExpressionKind::Text(v) if root => match v.insensitive {
                true => write!(w, "s.match_string::<true>({:?})", v.text)?,
                false => write!(w, "s.match_string::<false>({:?})", v.text)?,
            },
            ExpressionKind::Text(v) => match v.insensitive {
                true => write!(w, "builtin_text::<true>(s, {:?})", v.text)?,
                false => write!(w, "builtin_text::<false>(s, {:?})", v.text)?,
            },
            ExpressionKind::Regex(r) if root => {
                w.push_str("s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new(");
                write!(w, "{:?}", r.raw)?;
                w.push_str(").unwrap())})");
            }
            ExpressionKind::Regex(r) => {
                w.push_str("builtin_regex(s,{static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(||Regex::new(");
                write!(w, "{:?}", r.raw)?;
                w.push_str(").unwrap())})");
            }
            ExpressionKind::CharacterAny if root => w.push_str("s.match_char_if(|_| true)"),
            ExpressionKind::CharacterAny => w.push_str("builtin_any(s)"),
            ExpressionKind::CharacterRange(_) if root => {}
            ExpressionKind::CharacterRange(_) => {}
            ExpressionKind::Boolean(_) if root => {}
            ExpressionKind::Boolean(_) => {}
            ExpressionKind::Integer(_) if root => {}
            ExpressionKind::Integer(_) => {}
        }
        Ok(())
    }
}

pub struct ClassObject {
    name: String,
    derives: RuleDerive,
    file: FieldMap,
}
