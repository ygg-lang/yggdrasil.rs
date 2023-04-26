use super::*;
use itertools::Itertools;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilNamepath {
    pub identifiers: Vec<YggdrasilIdentifier>,
    pub range: Range<usize>,
}

impl Display for YggdrasilNamepath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let names = self.identifiers.iter().map(|s| s.text.as_str()).join("::");
        f.write_str(&names)
    }
}

impl PartialEq<str> for YggdrasilNamepath {
    fn eq(&self, other: &str) -> bool {
        match self.identifiers.as_slice() {
            [one] => one.text.eq(other),
            _ => false,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilIdentifier {
    pub text: String,
    pub range: Range<usize>,
}

impl Debug for YggdrasilIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier({:?}, {:?})", self.text, self.range)
    }
}

impl YggdrasilIdentifier {
    pub fn trim_underscore(&self) -> YggdrasilIdentifier {
        Self { text: self.text.trim_start_matches('_').to_string(), range: self.range.clone() }
    }
    pub fn is_ignore(&self) -> bool {
        self.text.starts_with('_')
    }
}
