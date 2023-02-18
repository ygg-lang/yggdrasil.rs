use super::*;
use crate::rule::YggdrasilNamepath;

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

impl From<YggdrasilIdentifier> for ExpressionNode {
    fn from(value: YggdrasilIdentifier) -> Self {
        let rule = RuleReference { name: value, boxed: false, inline: false };
        Self { kind: ExpressionKind::Rule(rule), tag: "".to_string() }
    }
}

impl From<RuleReference> for ExpressionNode {
    fn from(value: RuleReference) -> Self {
        Self { kind: ExpressionKind::Rule(value), tag: "".to_string() }
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
