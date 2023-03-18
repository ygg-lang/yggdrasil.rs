use crate::YggdrasilLanguage;
use core::{fmt::Debug, ops::Range};

/// A typed ast node
pub trait YggdrasilNode: Clone + Debug {
    type Language: YggdrasilLanguage;
    fn get_language(&self) -> Self::Language;
    fn get_range(&self) -> Range<usize>;
}
