use super::*;

impl<'i> RustWrite<'i> {
    pub fn language_name(&self) -> String {
        format!("{}Language", self.grammar.name)
    }
    pub fn rules(&self) -> impl Iterator<Item = GrammarRule> + '_ {
        self.grammar.rules.values().sorted().cloned()
    }
    pub fn ignore_rules(&self) -> impl Iterator<Item = GrammarRule> + '_ {
        self.rules().filter(|v| v.ignored)
    }
    pub fn ignore_rules_empty(&self) -> bool {
        self.ignore_rules().count() == 0
    }
    pub fn ignore_rule_pattern(&self) -> String {
        self.ignore_rules().map(|s| format!("Self::{}", s.safe_rule_name())).join(" | ")
    }
    pub fn ignore_rule_match(&self) -> String {
        let rules = self.ignore_rules().collect_vec();
        let mut out = String::new();
        match rules.as_slice() {
            [rule, rest @ ..] => {
                out.push_str(&format!("{}(s)", rule.parser_name()));
                for rule in rest {
                    out.push_str(&format!(".or_else(|s| {}(s))", rule.parser_name()));
                }
            }
            _ => {}
        }
        out
    }
}
impl<'i> RustWrite<'i> {
    pub fn rule_name(&self) -> String {
        format!("{}Rule", self.grammar.name)
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
