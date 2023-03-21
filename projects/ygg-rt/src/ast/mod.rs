use crate::YggdrasilRule;
use core::{fmt::Debug, ops::Range};

/// A typed ast node
pub trait YggdrasilNode: Clone + Debug {
    /// Specify the rules of this language
    type Rule: YggdrasilRule;
    /// get rule
    fn get_rule<R>(&self) -> Option<Self::Rule> {
        None
    }
    ///
    fn get_range(&self) -> Option<Range<usize>> {
        None
    }
}
