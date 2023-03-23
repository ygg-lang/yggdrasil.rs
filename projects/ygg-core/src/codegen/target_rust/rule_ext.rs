use convert_case::{Case, Casing};
use std::fmt::Debug;
use yggdrasil_ir::{
    nodes::{ExpressionBody, YggdrasilExpression, YggdrasilOperator},
    rule::YggdrasilIdentifier,
};

use super::*;

pub(super) trait RuleExt {
    fn parser_expression(&self) -> String;
}

impl RuleExt for GrammarRule {
    fn parser_expression(&self) -> String {
        let mut w = String::new();
        match &self.body {
            Some(s) => {
                if let Err(e) = s.write(&mut w, self, true) {
                    w.push_str(&format!("Err(/*{e}*/s)"))
                }
            }
            None => w.push_str("Err(/* empty node */s)"),
        }
        w
    }
}

trait NodeExt {
    fn write(&self, w: &mut String, ctx: &GrammarRule, root: bool) -> std::fmt::Result;
}

impl NodeExt for YggdrasilExpression {
    fn write(&self, w: &mut String, ctx: &GrammarRule, root: bool) -> std::fmt::Result {
        match &self.body {
            ExpressionBody::Choice(v) => {
                let (head, rest) = v.split();
                head.write(w, ctx, false)?;
                for pat in rest {
                    w.push_str(".or_else(|s|");
                    pat.write(w, ctx, false)?;
                    w.push_str(")");
                }
            }
            ExpressionBody::Concat(v) => {
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
            ExpressionBody::Unary(v) => {
                for o in &v.operators {
                    match o {
                        YggdrasilOperator::Negative => {
                            todo!()
                        }
                        YggdrasilOperator::Optional => w.push_str("s.optional(|s|"),
                        YggdrasilOperator::Repeats => write!(w, "s.repeat({}..{}, |s|", 0, u32::MAX)?,
                        YggdrasilOperator::Repeat1 => write!(w, "s.repeat({}..{}, |s|", 1, u32::MAX)?,
                        YggdrasilOperator::RepeatsBetween(min, max) => {
                            write!(w, "s.repeat({}..{}, |s|", min.unwrap_or(0), max.unwrap_or(u32::MAX))?
                        }
                        YggdrasilOperator::Boxing => {
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
            ExpressionBody::Ignored => w.push_str("builtin_ignore(s)"),
            ExpressionBody::Call(v) => write!(w, "Err(/*{}*/s)", v.name.to_string())?,
            ExpressionBody::Rule(r) => {
                let name = format!("parse_{}", r.name.text).to_case(Case::Snake);
                write!(w, "{name}(s)")?
            }
            ExpressionBody::Text(v) if root => write!(w, "s.match_string({:?}, {})", v.text, v.insensitive)?,
            ExpressionBody::Text(v) => write!(w, "builtin_text(s, {:?}, {})", v.text, v.insensitive)?,
            ExpressionBody::Regex(r) if root => {
                w.push_str("s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new(");
                write!(w, "{}", r)?;
                w.push_str(").unwrap())})");
            }
            ExpressionBody::Regex(r) => {
                w.push_str("builtin_regex(s,{static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(||Regex::new(");
                write!(w, "{}", r)?;
                w.push_str(").unwrap())})");
            }
            ExpressionBody::CharacterAny if root => w.push_str("s.match_char_if(|_| true)"),
            ExpressionBody::CharacterAny => w.push_str("builtin_any(s)"),
            ExpressionBody::CharacterRestOfLine => {}
            ExpressionBody::CharacterRange(_) if root => {}
            ExpressionBody::CharacterRange(_) => {}
            ExpressionBody::Boolean(_) if root => {}
            ExpressionBody::Boolean(_) => {}
            ExpressionBody::Integer(_) if root => {}
            ExpressionBody::Integer(_) => {}
        }
        match &self.tag {
            Some(s) => write!(w, ".and_then(|s| s.tag_node({:?}))", s.text.to_case(Case::Snake))?,
            None => {}
        }
        Ok(())
    }
}
