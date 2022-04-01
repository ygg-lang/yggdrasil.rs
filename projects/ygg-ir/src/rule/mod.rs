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
    /// 
    /// def rule {
    ///
    /// }
    ///
    /// def rule() -> char {
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
    /// def rule {
    ///
    /// }
    ///
    /// def inline rule {
    ///
    /// }
    ///
    /// def _rule {
    ///
    /// }
    /// ```
    pub auto_inline: bool,
    /// Automatically box when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #boxed(true)
    /// def rule {
    ///
    /// }
    ///
    /// def boxed rule {
    ///
    /// }
    /// ```
    pub auto_boxed: bool,
    ///
    pub auto_capture: bool,
    ///
    pub atomic: bool,
    /// The entry of the parser, the name of the parser to be exported
    pub entry: bool,
    /// Keep export this rule, even if this rule is not used
    pub keep: bool,
    ///
    pub body: ExpressionNode,

    /// position of all parts
    pub range: Range<usize>,
}

impl GrammarInfo {}
