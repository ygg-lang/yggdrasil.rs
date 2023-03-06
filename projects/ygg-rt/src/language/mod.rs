use crate::{errors::YggdrasilError, iterators::TokenTree, YggdrasilRule};

/// A trait with a single method that parses strings.
pub trait YggdrasilLanguage {
    /// Specify the rules of this language
    type Rule: YggdrasilRule;
    /// Parses a `&str` starting from `rule`.
    fn parse_cst(input: &str, rule: Self::Rule) -> Result<TokenTree<Self::Rule>, YggdrasilError<Self::Rule>>;
}
