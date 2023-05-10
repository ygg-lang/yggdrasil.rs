use crate::{errors::YggdrasilError, TokenStream, TokenTree, YggdrasilRule};

/// A trait with a single method that parses strings.
pub trait YggdrasilParser {
    /// Specify the rules of this language
    type Rule: YggdrasilRule;
    /// Get flatten token stream
    fn parse_lex(input: &str, rule: Self::Rule) -> Result<TokenStream<Self::Rule>, YggdrasilError<Self::Rule>> {
        Ok(Self::parse_cst(input, rule)?.flatten())
    }
    /// Parses a `&str` starting from `rule`.
    fn parse_cst(input: &str, rule: Self::Rule) -> Result<TokenTree<Self::Rule>, YggdrasilError<Self::Rule>>;
}
