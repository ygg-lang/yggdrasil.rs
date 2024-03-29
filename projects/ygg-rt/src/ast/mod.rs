use crate::{errors::ErrorKind, TextSpan, TokenPair, TokenTree, YggdrasilError, YggdrasilRule};
use alloc::format;
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
    /// from
    fn from_cst(mut tree: TokenTree<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        match tree.next() {
            Some(s) => Self::from_pair(s),
            None => Err(YggdrasilError::new_from_span(
                ErrorKind::CustomError { message: format!("no child: {}", tree.as_str()) },
                TextSpan { input: "", start: 0, end: 0 },
            )),
        }
    }
    /// from
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Err(YggdrasilError::new_from_span(
            ErrorKind::CustomError { message: format!("unimplemented {:?}", pair) },
            TextSpan { input: "", start: 0, end: 0 },
        ))
    }
}
