use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", Serialize, Deserialize)]
pub struct YggdrasilIdentifier {
    pub text: String,
    pub span: Range<usize>,
}

impl YggdrasilIdentifier {
    pub fn trim_underscore(&self) -> YggdrasilIdentifier {
        Self { text: self.text.trim_start_matches('_').to_string(), span: self.span.clone() }
    }
    pub fn is_ignore(&self) -> bool {
        self.text.starts_with('_')
    }
}
