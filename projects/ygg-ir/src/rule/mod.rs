use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Range,
};

use indexmap::set::IndexSet;
use serde::{Deserialize, Serialize};

use crate::*;

pub mod derive;
pub mod node;
pub mod parameter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionRule {}

/// Temporary value, will be removed after parsing
#[derive(Copy, Clone, Debug)]
pub struct GrammarRuleContext {
    pub capture: bool,
    pub atomic: bool,
}

#[derive(Clone, Debug)]
pub struct RuleParameter {
    pub kind: RuleParameterKind,
    pub name: String,
    pub typing: String,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RuleParameterKind {
    Optional,
    Required,
    Variadic,
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
    /// Whether the generated class is visible
    pub public: bool,
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
    /// The entry of the ast_mode, the name of the ast_mode to be exported
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
    ///
    pub body: ExpressionNode,
    /// position of all parts
    pub range: Range<usize>,
}

impl GrammarInfo {}
