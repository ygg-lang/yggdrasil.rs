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

impl From<YggdrasilIdentifier> for YggdrasilExpression {
    fn from(value: YggdrasilIdentifier) -> Self {
        let name = value.text.as_ref();
        let properties = &["XID_START", "XID_CONTINUE"];
        let out: ExpressionBody = match name {
            p if properties.contains(&p) => ExpressionBody::Regex(YggdrasilRegex::new(format!("[\\p{{{name}}}]"), 0..p.len())),
            "ANY" => ExpressionBody::CharacterAny.into(),
            "IGNORE" | "IGNORED" => ExpressionBody::Ignored.into(),
            "ASCII_DIGIT" => ExpressionBody::CharacterRange(RangeInclusive::new('0', '9')),
            _ => ExpressionBody::Rule(RuleReference::new(value)),
        };
        YggdrasilExpression { tag: None, remark: false, body: out }
    }
}

impl From<RuleReference> for YggdrasilExpression {
    fn from(value: RuleReference) -> Self {
        ExpressionBody::Rule(value).into()
    }
}
impl RuleReference {
    pub fn new(rule_name: YggdrasilIdentifier) -> Self {
        Self { name: rule_name.trim_underscore(), boxed: false, inline: rule_name.is_ignore() }
    }
}
