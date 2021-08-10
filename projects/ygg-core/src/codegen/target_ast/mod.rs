use crate::frontend::rule::ExpressionNode;
use yggdrasil_bootstrap::ast::Symbol;
use crate::frontend::{Map, Rule, RuleMethods};
use std::fmt::{Debug, Formatter, Write};
use std::fmt;

pub struct SymbolCountMap {
    rule: Rule,
    map: Map<String, SymbolCount>,
}


pub enum SymbolCount {
    One(String, String),
    Some(String, String),
    Many(String, String),
}

impl From<Rule> for SymbolCountMap {
    fn from(rule: Rule) -> Self {
        let mut out = Self {
            rule,
            map: Default::default(),
        };
        out.count();
        return out;
    }
}


impl SymbolCountMap {
    fn count(&mut self) {}
    fn one(&mut self, s: &Symbol) {}
    fn some(&mut self, s: &Symbol) {}
    fn many(&mut self, s: &Symbol) {}
}

impl ExpressionNode {
    fn count(&self, map: &mut SymbolCountMap) {
        self.expression
    }
}

impl SymbolCountMap {
    fn write_struct(&self, f: &mut Formatter<'_>) -> fmt::Result {

        writeln!(f, "pub struct {} {{", self.name)?;
        for symbol in self.map.values() {
            f.write_str("    pub ");
            symbol.write_struct(f)?;
        }
        writeln!(f, "    pub range: (usize, usize),")?;
        writeln!(f, "}}")
    }
    fn write_string_node_parsing(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "string_node!(Node, {});", self.name)
    }

}

impl RuleMethods {
    fn write_derive(&self, f: &mut Formatter<'_>) -> fmt::Result {
        auto_derive


        writeln!(f, "string_node!(Node, {});", self.name)
    }
    fn write_custom_derive(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.debug {
            None => {}
            Some(_) => {}
        }
        writeln!(f, "#[derive({})]", self.auto_derive.join(", "))?;
        writeln!(f, "string_node!(Node, {});", self.name)
    }
}

impl SymbolCount {
    fn write_struct(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match ty {
            Self::One(s, t) => { writeln!(f, "{}: {},", s, t) }
            Self::Some(s, t) => { writeln!(f, "{}: Option<{}>,", s, t) }
            Self::Many(s, t) => { writeln!(f, "{}: Vec<{}>,", s, t) }
        }
    }
}

