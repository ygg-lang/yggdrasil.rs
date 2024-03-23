use crate::{errors::YggdrasilErrorKind, TextSpan, TokenPair, TokenTree, YggdrasilError, YggdrasilRule};
use alloc::format;
use core::{fmt::Debug, ops::Range};

/// A typed ast node
pub trait YggdrasilNode<'i>: Clone + Debug {
    /// Specify the rules of this language
    type Rule: YggdrasilRule;
    /// get rule
    fn get_rule<R>(&self) -> Option<Self::Rule> {
        None
    }
    ///
    fn get_range(&self) -> Range<usize> {
        0..0
    }

    fn from_str(input: &'i str) -> Result<Self, YggdrasilError<Self::Rule>> {
        Err(YggdrasilError::new_from_span(
            YggdrasilErrorKind::CustomError { message: format!("unimplemented parse from") },
            TextSpan { input: "", start: 0, end: 0 },
        ))
    }

    /// from
    fn from_cst(mut tree: TokenTree<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        match tree.next() {
            Some(s) => Self::from_pair(s),
            None => Err(YggdrasilError::new_from_span(
                YggdrasilErrorKind::CustomError { message: format!("no child: {}", tree.as_str()) },
                TextSpan { input: "", start: 0, end: 0 },
            )),
        }
    }
    /// from
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Err(YggdrasilError::new_from_span(
            YggdrasilErrorKind::CustomError { message: format!("unimplemented {:?}", pair) },
            TextSpan { input: "", start: 0, end: 0 },
        ))
    }
}
