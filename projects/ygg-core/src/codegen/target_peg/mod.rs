use crate::frontend::{
    rule::{ChoiceExpression, ConcatExpression, DataKind, ExpressionKind, ExpressionNode, UnaryExpression},
    GrammarInfo, GrammarRule,
};
use std::fmt::{Arguments, Display, Formatter, Write};

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

impl GrammarInfo {
    fn write_peg(&self, w: &mut PegBuffer) {
        for (_, rule) in &self.rules {
            rule.write_peg(w, self).unwrap_or_default()
        }
    }
}

impl GrammarRule {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        if self.atomic_rule {
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

impl ExpressionNode {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        w.tag(&self.tag);
        w.write_start();
        match &self.kind {
            ExpressionKind::Unary(expr) => expr.write_peg(w, info)?,
            ExpressionKind::Choice(expr) => expr.write_peg(w, info)?,
            ExpressionKind::Concat(expr) => expr.write_peg(w, info)?,
            ExpressionKind::Data(expr) => expr.write_peg(w, info)?,
        }
        w.write_end();
        Ok(())
    }
}

impl DataKind {
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
                write!(w, "{}{}{}", info.rule_prefix, r.name, info.rule_suffix)?;
            }
        }
        Ok(())
    }
}

impl UnaryExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        unimplemented!()
    }
}

impl ChoiceExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        for (id, expr) in self.inner.iter().enumerate() {
            if id != 0 {
                w.write_char('|')?;
            }
            w.write_start();
            expr.write_peg(w, info)?;
            w.write_end();
        }

        Ok(())
    }
}

impl ConcatExpression {
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
