use super::*;

impl SymbolCountMap {
    pub(super) fn write_struct(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.rule.custom_methods.write_derive(f)?;
        writeln!(f, "pub struct {} {{", self.rule.name.data)?;
        for symbol in self.map.values() {
            f.write_str("    pub ");
            symbol.write_struct(f)?;
        }
        f.write_str("    pub ");
        writeln!(f, "range: (usize, usize),")?;
        writeln!(f, "}}")
    }
}

impl SymbolCount {
    fn write_struct(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::One(s, t) => {
                writeln!(f, "{}: {},", s, t)
            }
            Self::Some(s, t) => {
                writeln!(f, "{}: Option<{}>,", s, t)
            }
            Self::Many(s, t) => {
                writeln!(f, "{}: Vec<{}>,", s, t)
            }
        }
    }
}

impl RuleMethods {
    fn write_derive(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut auto_derive = vec![];
        auto_derive.push("Clone");
        if self.debug.is_none() {
            auto_derive.push("Debug")
        }
        writeln!(f, "#[derive({})]", auto_derive.join(", "))
    }
}