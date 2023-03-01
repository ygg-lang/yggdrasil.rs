use super::*;

impl YggdrasilExpression {
    pub fn builtin(rule: &RuleReference) -> Option<Self> {
        let name = rule.name.text.as_ref();
        let properties = &["XID_START", "XID_CONTINUE"];
        let out = match name {
            p if properties.contains(&p) => YggdrasilRegex::new(format!("[\\p{{{name}}}]"), 0..p.len()).into(),
            "ASCII_DIGIT" => RangeInclusive::new('0', '9').into(),
            "ANY" => ExpressionKind::CharacterAny.into(),
            _ => return None,
        };
        Some(out)
    }
}
