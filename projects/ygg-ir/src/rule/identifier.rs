use std::path::Path;

use itertools::Itertools;
use yggdrasil_error::Url;

use yggdrasil_error::FileSpan;

use super::*;

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
    pub span: FileSpan,
}

impl Debug for YggdrasilIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier({:?}, {:?})", self.text, self.span.get_range())
    }
}

impl YggdrasilIdentifier {
    pub fn trim_underscore(&self) -> YggdrasilIdentifier {
        Self { text: self.text.trim_start_matches('_').to_string(), span: self.span.clone() }
    }
    pub fn with_range(mut self, range: &Range<usize>) -> Self {
        self.span.set_range(range.clone());
        self
    }
    pub fn with_local(mut self, path: &Path) -> Self {
        match path.canonicalize() {
            Ok(o) => match Url::from_file_path(o) {
                Ok(o) => self.set_remote(o),
                Err(_) => {}
            },
            Err(_) => {}
        }
        self
    }
    pub fn set_remote(&mut self, _link: Url) {
        // self.span.set_file(link.into())
    }
    pub fn is_ignore(&self) -> bool {
        self.text.starts_with('_')
    }
}
