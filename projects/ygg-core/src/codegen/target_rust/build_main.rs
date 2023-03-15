use super::*;
use convert_case::{Case, Casing};
use std::collections::BTreeMap;
use yggdrasil_ir::rule::{GrammarRuleKind, YggdrasilField, YggdrasilVariant};

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
            write!(w, "pub struct {} {{", rule.structure_name())?;
            for (key, value) in fields {
                writeln!(w, "    {}: {},", key, value.name)?
            }
            w.push_str("}\n");
        };
        w
    }
}
