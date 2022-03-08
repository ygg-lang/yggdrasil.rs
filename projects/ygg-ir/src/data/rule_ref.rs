use super::*;

//
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RuleReference {
    pub tag: String,
    pub name: String,
    pub boxed: bool,
    pub inline: bool,
}

impl Display for RuleReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.boxed {
            write!(f, "@box(")?
        }
        if !self.tag.is_empty() {
            write!(f, "{}:", self.tag)?
        }
        if self.inline {
            write!(f, "_")?
        }
        write!(f, "{}", self.name)?;
        if self.boxed {
            write!(f, ")")?
        }
        Ok(())
    }
}

impl RuleReference {
    pub fn new(name: &str) -> Self {
        Self { tag: "".to_string(), name: name.trim_start_matches("_").to_string(), boxed: false, inline: name.starts_with('_') }
    }
}
