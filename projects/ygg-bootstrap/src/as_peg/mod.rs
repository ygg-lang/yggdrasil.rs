use std::fmt::{Arguments, Display, Formatter, Write};

use peginator::PegParser;

use crate::ygg::{DefineStatement, Identifier, Program, Statement};

mod extend;

pub struct PegBuffer {
    buffer: String,
}

impl PegBuffer {
    pub fn write_str(&mut self, str) {

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
                Statement::DefineStatement(define) => {
                    define.write_peg(cfg)?
                }
                Statement::EmptyStatement(_) => {}
            }
        }
        Ok(())
    }
}

impl WritePeg for DefineStatement {
    fn write_peg(&self, cfg: &mut PegBuffer) -> std::fmt::Result {
        write!(cfg, "{}", self.symbol)
    }
}


pub fn as_peg(ygg: &str) -> String {
    let pro = Program::parse(ygg).unwrap();
    println!("{:#?}", pro);
    let mut cfg = PegBuffer {
        buffer: "".to_string()
    };
    pro.write_peg(&mut cfg).unwrap();
    return cfg.buffer.to_owned();
}