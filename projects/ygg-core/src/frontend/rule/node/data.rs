use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum DataKind {
    AnyCharacter,
    String(String),
    Regex(String),
    Rule(RuleReference),
    Integer(isize),
    Character(char),
    CharacterRange(Range<char>),
    CharacterSet(CharacterSet),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RuleReference {
    pub name: String,
    pub inline: bool,
}

impl RuleReference {
    pub fn new(name: &str) -> Self {
        Self { name: name.trim_start_matches("_").to_string(), inline: name.starts_with('_') }
    }
}

impl ExpressionKind {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::AnyCharacter,
            _ => DataKind::Rule(RuleReference::new(name)),
        };
        ExpressionKind::Data(Box::new(data))
    }
    pub fn string(string: String) -> Self {
        let data = DataKind::String(string);
        ExpressionKind::Data(Box::new(data))
    }
}

impl From<DataKind> for ExpressionKind {
    fn from(e: DataKind) -> Self {
        Self::Data(Box::new(e))
    }
}
