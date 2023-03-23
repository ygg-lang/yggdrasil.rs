use crate::{errors::ErrorKind, iterators::TokenPair, TextSpan, TokenTree, YggdrasilError, YggdrasilRule};
use alloc::{format, string::ToString};
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
    fn from_cst(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Err(YggdrasilError::new_from_span(
            ErrorKind::CustomError { message: format!("unimplemented {:?}", pair.as_rule()) },
            pair.as_span(),
        ))
    }
}
