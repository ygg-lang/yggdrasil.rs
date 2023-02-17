pub use self::{
    derive::RuleDerive,
    parameter::{RuleParameter, RuleParameterKind},
};
use crate::{
    grammar::GrammarInfo,
    nodes::{ExpressionKind, ExpressionNode, Operator},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    ops::{BitAnd, BitOr, Range},
};

pub mod derive;
pub mod parameter;

mod identifier;

pub use self::identifier::{YggdrasilIdentifier, YggdrasilNamepath};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionRule {}

/// Temporary value, will be removed after parsing
#[derive(Copy, Clone, Debug)]
pub struct GrammarRuleContext {
    pub capture: bool,
    pub atomic: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GrammarRuleKind {
    Class,
    Union,
    Climb,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", Serialize, Deserialize)]
pub struct GrammarRule {
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def RuleName {
    ///
    /// }
    /// ```
    pub name: YggdrasilIdentifier,
    /// Kind of this rule
    pub kind: GrammarRuleKind,
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
    /// Ignore this node in ast mode.
    ///
    /// ## Examples
    /// ```ygg
    /// #ignore(true)
    /// ignore class Rule { }
    /// ```
    pub ignored: bool,
    ///
    pub body: ExpressionNode,
    /// position of all parts
    pub range: Range<usize>,
}

impl Ord for GrammarRule {
    fn cmp(&self, other: &Self) -> Ordering {
        other.name.text.cmp(&other.name.text)
    }
}

impl PartialOrd for GrammarRule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.name.text.partial_cmp(&other.name.text)
    }
}

impl GrammarInfo {}

impl GrammarRule {
    pub fn new(name: YggdrasilIdentifier, range: &Range<usize>, kind: GrammarRuleKind) -> Self {
        Self {
            name,
            r#type: String::new(),
            document: String::new(),
            public: false,
            derives: RuleDerive::default(),
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            ignored: false,
            kind,
            body: ExpressionNode::empty(),
            range: range.clone(),
        }
    }
}
