use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", Serialize, Deserialize)]
pub struct YggdrasilIdentifier {
    pub name: String,
    pub span: Range<usize>,
}

impl YggdrasilIdentifier {
    pub fn trim_underscore(&self) -> YggdrasilIdentifier {
        Self { name: self.name.trim_start_matches('_').to_string(), span: self.span.clone() }
    }
    pub fn is_ignore(&self) -> bool {
        self.name.starts_with('_')
    }
}
