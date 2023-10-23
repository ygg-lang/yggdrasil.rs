use super::*;
use url::Url;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FileSpan {
    pub path: Option<Url>,
    pub range: Range<usize>,
}
