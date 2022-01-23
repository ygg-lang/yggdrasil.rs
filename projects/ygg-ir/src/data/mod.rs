use ucd_trie::TrieSetOwned;

use crate::rule::ExpressionKind;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum DataKind {
    AnyCharacter,
    Integer(isize),
    String(String),
    Rule(RuleReference),
    CharacterSet(TrieSetOwned),
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
            "XID_START" => DataKind::Builtin(name.to_string()),
            _ => DataKind::Rule(RuleReference::new(name)),
        };
        ExpressionKind::Data(Box::new(data))
    }
    pub fn string(string: String) -> Self {
        let data = DataKind::String(string);
        ExpressionKind::Data(Box::new(data))
    }
    pub fn builtin(name: &str) -> Option<Self> {}
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
            DataKind::Builtin(_) => {}
        }
    }
}
