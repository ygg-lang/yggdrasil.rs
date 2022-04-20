use proc_macro2::TokenStream;
use std::{
    fmt::{Arguments, Display, Formatter, Write},
    mem::take,
};
use yggdrasil_error::{Diagnostic, YggdrasilResult};
use yggdrasil_ir::{
    ChoiceExpression, CodeGenerator, ConcatExpression, DataKind, ExpressionKind, GrammarInfo, GrammarRule, Operator,
    UnaryExpression,
};

mod build_rust;
mod build_symbol;

pub fn as_peg(grammar: &GrammarInfo) -> String {
    let mut buffer = PegBuffer { buffer: "".to_string(), indent: 0 };
    grammar.write_peg(&mut buffer);
    buffer.buffer
}

struct PegBuffer {
    buffer: String,
    indent: usize,
}

impl CodeGenerator for PegBuffer {
    type Output = String;

    fn generate(&mut self, info: &GrammarInfo) -> YggdrasilResult<Self::Output> {
        let mut errors = vec![];
        for (_, rule) in &info.rules {
            rule.write_peg(self, info)?
        }
        Ok(Diagnostic { success: take(&mut self.buffer), errors })
    }
}

trait WritePeg {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result;
}

impl WritePeg for GrammarRule {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        if self.atomic {
            w.write_str("@no_skip_ws\n")?
        }
        match self.r#type.to_ascii_lowercase().as_str() {
            "string" => {
                w.write_str("@string\n")?;
                w.write_str("@position\n")?;
            }
            "char" | "character" => {
                w.write_str("@char\n")?;
            }
            _ => {}
        }
        write!(w, "{}{}{} = ", info.rule_prefix, self.name, info.rule_suffix)?;
        self.body.write_peg(w, info)?;
        w.semicolon();
        Ok(())
    }
}
impl WritePeg for ExpressionKind {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        match self {
            ExpressionKind::Unary(expr) => expr.write_peg(w, info),
            ExpressionKind::Choice(expr) => expr.write_peg(w, info),
            ExpressionKind::Concat(expr) => expr.write_peg(w, info),
            ExpressionKind::Data(expr) => expr.write_peg(w, info),
        }
    }
}

impl WritePeg for DataKind {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        match self {
            DataKind::AnyCharacter => {
                w.write_str("char")?;
            }
            DataKind::String(s) => {
                w.write_char('"')?;
                w.write_str(s)?;
                w.write_char('"')?;
            }
            DataKind::Regex(_) => {
                unimplemented!()
            }
            DataKind::Integer(_) => {
                unimplemented!()
            }
            DataKind::Character(c) => w.char_token(*c),
            DataKind::CharacterRange(r) => {
                w.char_token(r.start);
                w.write_str("..")?;
                w.char_token(r.end);
            }
            DataKind::CharacterSet(_) => {
                unimplemented!()
            }
            DataKind::Rule(r) => {
                w.tag(&r.tag);
                write!(w, "{}{}{}", info.rule_prefix, r.name, info.rule_suffix)?;
            }
            DataKind::Builtin(r) => w.write_str(r)?,
        }
        Ok(())
    }
}

impl WritePeg for UnaryExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        let mut pre = vec![];
        let mut post = vec![];
        for op in &self.ops {
            match op {
                Operator::Negative => {
                    pre.push("!(");
                    post.push(")")
                }
                Operator::Optional => {
                    pre.push("[");
                    post.push("]")
                }
                Operator::Repeats => {
                    pre.push("{");
                    post.push("}")
                }
                Operator::Repeat1 => {
                    pre.push("{");
                    post.push("}+")
                }
                Operator::RepeatsBetween(_, _) => {
                    pre.push("(");
                    post.push(")")
                }
                Operator::Remark => {
                    pre.push("(");
                    post.push(")")
                }
                Operator::Recursive => {
                    pre.push("(");
                    post.push(")")
                }
            }
        }
        for s in pre {
            w.write_str(s)?
        }
        self.base.write_peg(w, info)?;
        for s in post {
            w.write_str(s)?
        }
        Ok(())
    }
}

impl WritePeg for ChoiceExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        for (id, expr) in self.inner.iter().enumerate() {
            if id != 0 {
                w.write_char('|')?;
            }
            // w.write_start();
            expr.write_peg(w, info)?;
            // w.write_end();
        }

        Ok(())
    }
}

impl WritePeg for ConcatExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        for (index, expr) in self.sequence.iter().enumerate() {
            if index != 0 {
                w.write_char('|')?;
            }
            expr.write_peg(w, info)?;
        }
        Ok(())
    }
}
