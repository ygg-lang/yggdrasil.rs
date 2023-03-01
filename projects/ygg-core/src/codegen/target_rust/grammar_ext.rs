use super::*;

pub trait GrammarExt {
    fn language_name(&self) -> String;
    fn rule_name(&self) -> String;
    fn rules(&self) -> Vec<GrammarRule>;
    fn ignore_rules(&self) -> Vec<GrammarRule>;
    fn ignore_rules_empty(&self) -> bool;
    fn ignore_rule_pattern(&self) -> String;
    fn ignore_rule_match(&self) -> String;
}

impl GrammarExt for GrammarInfo {
    fn language_name(&self) -> String {
        format!("{}Language", self.name.text)
    }
    fn rule_name(&self) -> String {
        format!("{}Rule", self.name.text)
    }
    fn rules(&self) -> Vec<GrammarRule> {
        self.rules.values().sorted().cloned().collect_vec()
    }
    fn ignore_rules(&self) -> Vec<GrammarRule> {
        self.rules.values().sorted().filter(|v| v.ignored).cloned().collect_vec()
    }
    fn ignore_rules_empty(&self) -> bool {
        false
    }
    fn ignore_rule_pattern(&self) -> String {
        let mut out = String::from("Self::IgnoreText | Self::IgnoreRegex");
        for ignore in self.ignore_rules() {
            out.push_str(&format!(" | Self::{}", ignore.safe_rule_name()))
        }
        out
    }
    fn ignore_rule_match(&self) -> String {
        let rules = self.ignore_rules();
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
