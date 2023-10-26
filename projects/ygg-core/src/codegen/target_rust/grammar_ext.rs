use super::*;
use crate::codegen::target_rust::filters::safe_rust_id;

pub trait GrammarExt {
    fn rule_name(&self) -> String;
    fn ignore_rules(&self) -> Vec<GrammarRule>;
    fn ignore_rules_empty(&self) -> bool;
    fn ignore_rule_pattern(&self) -> String;
    fn ignore_rule_match(&self) -> String;
}

impl GrammarExt for GrammarInfo {
    fn rule_name(&self) -> String {
        format!("{}Rule", self.name.text)
    }
    fn ignore_rules(&self) -> Vec<GrammarRule> {
        self.rules.values().sorted().filter(|v| v.hidden).cloned().collect_vec()
    }
    fn ignore_rules_empty(&self) -> bool {
        false
    }
    fn ignore_rule_pattern(&self) -> String {
        let mut out = String::from("Self::HiddenTex");
        for ignore in self.ignore_rules() {
            out.push_str(&format!(" | Self::{}", safe_rust_id(&ignore.name.text).unwrap()))
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
