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
    pub tag: String,
    pub name: String,
    pub inline: bool,
}

impl From<DataKind> for ExpressionKind {
    fn from(e: DataKind) -> Self {
        Self::Data(Box::new(e))
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

impl RuleReference {
    pub fn new(name: &str) -> Self {
        Self { tag: "".to_string(), name: name.trim_start_matches("_").to_string(), inline: name.starts_with('_') }
    }
}

impl DataKind {
    pub fn set_tag(&mut self, tag: String) {
        match self {
            DataKind::AnyCharacter => {}
            DataKind::String(_) => {}
            DataKind::Regex(_) => {}
            DataKind::Rule(r) => r.tag = tag,
            DataKind::Integer(_) => {}
            DataKind::Character(_) => {}
            DataKind::CharacterRange(_) => {}
            DataKind::CharacterSet(_) => {}
        }
    }
}
