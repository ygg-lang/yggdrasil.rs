use super::*;

impl PestCSTW {
    pub(super) fn write_rule(&self, f: &mut impl Write) -> std::fmt::Result {
        writeln!(f, "use super::*;")?;
        writeln!(f)?;
        self.write_enum(f)?;
        writeln!(f)?;
        self.write_parser(f)?;
        writeln!(f)
    }
    fn write_enum(&self, f: &mut impl Write) -> std::fmt::Result {
        writeln!(f, "#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]")?;
        writeln!(f, "pub enum Rule {{")?;
        for rule in self.rules {
            writeln!(f, "    {}", rule)?;
        }
        writeln!(f, "}}")
    }
    fn write_parser(&self, f: &mut impl Write) -> std::fmt::Result {
        writeln!(
            f,
            r#"
impl Parser<Rule> for CSTBuilder {{
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {{
        pest::state(input, |state| match rule {{
"#
        )?;
        for rule in self.rules {
            f.write_str(&" ".repeat(12))?;
            writeln!(f, "Rule::{rule} => parse::{rule}(state),", rule= rule)?;
        }
        writeln!(
            f,
            r#"
            _ => unreachable!("cannot start with such rule {{:?}}", rule),
        }})
    }}
}}
"#
        )
    }
}