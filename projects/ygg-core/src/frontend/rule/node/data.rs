use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DataExpression {
    pub tag: String,
    pub kind: DataKind,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum DataKind {
    AnyCharacter,
    String(String),
    Regex(String),
    Rule(bool, String),
    Integer(isize),
    Character(char),
    CharacterRange(Range<char>),
    CharacterSet(CharacterSet),
}

impl DataExpression {
    pub fn rule(name: &str, tag: &str) -> Self {
        let kind = DataKind::Rule(name.starts_with('_'), name.trim_start_matches("_").to_string());
        Self { tag: tag.to_string(), kind }
    }
}

impl From<DataExpression> for Expression {
    fn from(e: DataExpression) -> Self {
        Self::Data(Box::new(e))
    }
}
