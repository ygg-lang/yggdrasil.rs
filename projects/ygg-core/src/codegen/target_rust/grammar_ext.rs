use convert_case::{Case, Casing};

use super::*;

pub(super) trait GrammarExt {
    fn language_name(&self) -> String;
}

impl GrammarExt for GrammarInfo {
    fn language_name(&self) -> String {
        format!("{}Language", self.name)
    }
}

impl Write for RustCodegen {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}
