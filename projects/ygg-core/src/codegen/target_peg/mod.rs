use crate::frontend::{
    rule::{ConcatExpression, DataKind, Expression, UnaryExpression},
    GrammarInfo, GrammarRule,
};
use std::fmt::{Arguments, Display, Formatter, Write};
mod build_symbol;

pub fn as_peg(grammar: &GrammarInfo) -> String {
    let mut buffer = PegBuffer { buffer: "".to_string(), indent: 0 };
    match grammar.write_peg(&mut buffer) {
        Ok(_) => {}
        Err(_) => {}
    }
    buffer.buffer
}

struct PegBuffer {
    buffer: String,
    indent: usize,
}

impl GrammarInfo {
    fn write_peg(&self, w: &mut PegBuffer) -> std::fmt::Result {
        for (_, rule) in &self.rules {
            rule.write_peg(w, self)
        }
        Ok(())
    }
}

impl GrammarRule {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        write!(w, "{}{}{} = ", info.rule_prefix, self.name, info.rule_suffix)?;
        self.body.write_peg(w, info)?;
        w.write_semicolon();
        Ok(())
    }
}

impl Expression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        self.write_tag(w);
        w.write_start();
        match self {
            Expression::Unary(expr) => expr.write_peg(w, info)?,
            Expression::Choice(expr) => expr.write_peg(w, info)?,
            Expression::Concat(expr) => expr.write_peg(w, info)?,
            Expression::Data(expr) => expr.kind.write_peg(w, info)?,
        }
        w.write_end();
        Ok(())
    }
    fn write_tag(&self, w: &mut PegBuffer) {
        match self {
            Expression::Unary(expr) => w.write_tag(&expr.tag),
            Expression::Choice(expr) => w.write_tag(&expr.tag),
            Expression::Concat(expr) => w.write_tag(&expr.tag),
            Expression::Data(expr) => w.write_tag(&expr.tag),
        }
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
            DataKind::Regex(_) => {}
            DataKind::Integer(_) => {}
            DataKind::Character(c) => {
                if c == '\'' {
                    write!(w, "\"'\"")?
                }
                else {
                    write!(w, "'{}'", c)?
                }
                write!(w, "'{}'", c)?
            }
            DataKind::CharacterRange(_) => {}
            DataKind::CharacterSet(_) => {}
        }

        Ok(())
    }
}

impl UnaryExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        Ok(())
    }
}

impl ConcatExpression {
    fn write_peg(&self, w: &mut PegBuffer, info: &GrammarInfo) -> std::fmt::Result {
        w.write_char('(')?;
        self.base.write_peg(w, info)?;
        w.write_char(')')?;
        for (ws, expr) in &self.rest {
            w.write_str(" | (")?;
            if ws {
                expr.write_peg(w, info)?;
            }
            else {
                expr.write_peg(w, info)?;
            }
            w.write_char(')')?;
        }
        Ok(())
    }
}
