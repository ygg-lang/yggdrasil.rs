use super::*;

//
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", Serialize, Deserialize)]
pub struct RuleReference {
    pub name: YggdrasilIdentifier,
    pub boxed: bool,
    pub inline: bool,
}

impl Display for RuleReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.boxed {
            write!(f, "@box(")?
        }
        if self.inline {
            write!(f, "_")?
        }
        write!(f, "{}", self.name.text)?;
        if self.boxed {
            write!(f, ")")?
        }
        Ok(())
    }
}

impl RuleReference {
    pub fn new(rule_name: YggdrasilIdentifier) -> Self {
        Self { name: rule_name.trim_underscore(), boxed: false, inline: rule_name.is_ignore() }
    }

    pub fn to_node<S>(self, tag: S) -> ExpressionNode
    where
        S: Into<String>,
    {
        ExpressionNode { tag: tag.into(), kind: ExpressionKind::Rule(self) }
    }
}
