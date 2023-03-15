use super::*;
use convert_case::{Case, Casing};
use std::collections::BTreeMap;
use yggdrasil_ir::rule::{FieldKind, GrammarRuleKind, YggdrasilField, YggdrasilVariant};

impl<'i> RustWriteMain<'i> {
    pub fn write_structures(&self) -> Vec<String> {
        let mut out = vec![];
        for rule in self.grammar.rules.values() {
            match rule.kind {
                GrammarRuleKind::Class => out.push(self.write_class(&rule, rule.class_fields())),
                GrammarRuleKind::Union => {}
                GrammarRuleKind::Climb => {}
            }
        }
        return out;
    }

    pub fn write_class(&self, rule: &GrammarRule, fields: BTreeMap<String, YggdrasilField>) -> String {
        let mut w = String::new();
        let _: std::fmt::Result = try {
            writeln!(w, "pub struct {} {{", rule.node_name())?;
            for (key, value) in fields {
                match &value.kind {
                    FieldKind::Rule(name) => match self.grammar.rules.get(name) {
                        Some(s) => writeln!(w, "    pub {}: {},", key, s.node_name())?,
                        None => writeln!(w, "    // Missing rule {}", key)?,
                    },
                    FieldKind::IgnoreText => {}
                    FieldKind::IgnoreRegex => {}
                    FieldKind::IgnoreComment => {}
                    FieldKind::IgnoreWhitespace => {}
                }
            }
            writeln!(w, "    pub range: Range<usize>,")?;
            w.push_str("}\n");
        };
        w
    }
}
