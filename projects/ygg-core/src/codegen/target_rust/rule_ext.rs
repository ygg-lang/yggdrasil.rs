use convert_case::{Case, Casing};
use std::fmt::Debug;
use yggdrasil_ir::nodes::{ExpressionKind, YggdrasilExpression, YggdrasilOperator};

use super::*;

pub(super) trait RuleExt {
    fn parser_expression(&self) -> String;
}

impl RuleExt for GrammarRule {
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
            ExpressionKind::Call(_) => w.push_str("parse_Function(s)"),
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
            ExpressionKind::CharacterRestOfLine => {}
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
