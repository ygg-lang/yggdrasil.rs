use std::fmt::{Arguments, Display, Formatter, Write};

use peginator::PegParser;

use crate::ygg::{DefineStatement, Identifier, Program, Statement};

mod extend;

pub struct PegBuffer {
    buffer: String,
    indent: usize,
    capture: bool,
}

impl PegBuffer {
    pub fn indent(&mut self) {
        self.indent = self.indent.saturating_add(4)
    }
    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(4)
    }
    pub fn newline(&mut self) {
        self.buffer.push('\r');
        self.buffer.push_str(&" ".repeat(self.indent))
    }
}

impl Write for PegBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}

trait WritePeg {
    fn write_peg(&self, cfg: &mut PegBuffer) -> std::fmt::Result;
}

impl WritePeg for Program {
    fn write_peg(&self, cfg: &mut PegBuffer) -> std::fmt::Result {
        for stmt in &self.statement {
            match stmt {
                Statement::DefineStatement(define) => define.write_peg(cfg)?,
                Statement::EmptyStatement(_) => {}
            }
        }
        Ok(())
    }
}

impl WritePeg for DefineStatement {
    fn write_peg(&self, cfg: &mut PegBuffer) -> std::fmt::Result {
        write!(cfg, "{}", self.symbol)?;
        cfg.newline();

        Ok(())
    }
}

pub fn as_peg(ygg: &str) -> String {
    let pro = Program::parse(ygg).unwrap();
    let mut cfg = PegBuffer { indent: 0, buffer: String::new(), capture: true };
    pro.write_peg(&mut cfg).unwrap();
    return cfg.buffer.to_owned();
}
