use crate::*;
use indexmap::set::IndexSet;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Range,
};

pub mod derive;
pub mod node;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionRule {}

/// Temporary value, will be removed after parsing
#[derive(Copy, Clone, Debug)]
pub struct GrammarRuleContext {
    pub capture: bool,
    pub atomic: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GrammarRule {
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def RuleName {
    ///
    /// }
    /// ```
    pub name: String,
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def rule -> char {
    ///
    /// }
    ///
    /// def rule() -> char {
    ///
    /// }
    /// ```
    pub r#type: String,
    /// Document of this rule
    ///
    /// ## Examples
    /// ```ygg
    /// /// this is documents
    /// def rule {
    ///
    /// }
    /// ```
    pub document: String,
    ///
    pub derives: RuleDerive,
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #inline(true)
    /// def Rule {
    ///
    /// }
    ///
    /// def inline Rule {
    ///
    /// }
    ///
    /// def _Rule {
    ///
    /// }
    /// ```
    pub auto_inline: bool,
    /// Automatically box when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #boxed(true)
    /// def Rule {
    ///
    /// }
    ///
    /// def boxed Rule {
    ///
    /// }
    /// ```
    pub auto_boxed: bool,
    /// The entry of the parser, the name of the parser to be exported
    ///
    /// ## Examples
    /// ```ygg
    /// #entry(true)
    /// def Rule {
    ///
    /// }
    ///
    /// def entry Rule {
    ///
    /// }
    /// ```
    pub entry: bool,
    /// union struct
    ///
    /// ## Examples
    /// ```ygg
    /// union Rule {
    ///
    /// }
    /// ```
    pub union: bool,
    /// Keep export this rule, even if this rule is not used
    /// ## Examples
    /// ```ygg
    /// class keep Rule {
    ///
    /// }
    /// ```
    pub force_export: bool,
    ///
    pub body: ExpressionNode,
    /// position of all parts
    pub range: Range<usize>,
}

impl GrammarInfo {}
