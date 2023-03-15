use crate::YggdrasilLanguage;
use core::ops::Range;

/// A typed ast node
pub trait YggdrasilNode {
    type Language: YggdrasilLanguage;
    fn get_language(&self) -> Self::Language;
    fn get_range(&self) -> Range<usize>;
}
