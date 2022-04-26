use super::*;

impl Write for PegCodegen {
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

impl PegCodegen {
    pub fn write_start(&mut self) {
        self.buffer.push_str("(")
    }
    pub fn write_end(&mut self) {
        self.buffer.push_str(")")
    }
    pub fn semicolon(&mut self) {
        self.buffer.push_str(";\n\n")
    }
    pub fn tag(&mut self, tag: &str) {
        if tag.is_empty() {
            return;
        }
        else {
            self.buffer.push_str(&tag);
            self.buffer.push(':')
        }
    }
    pub fn char_token(&mut self, token: char) {
        if token == '\'' {
            self.buffer.push_str("\"'\"");
        }
        else {
            self.buffer.push('\'');
            self.buffer.push(token);
            self.buffer.push('\'');
        }
    }
}
