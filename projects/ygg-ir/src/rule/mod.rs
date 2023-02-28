pub use self::{
    derive::RuleDerive,
    parameter::{RuleParameter, RuleParameterKind},
};
use crate::{
    grammar::GrammarInfo,
    nodes::{ExpressionKind, YggdrasilExpression, YggdrasilOperator},
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

mod modifiers;

pub use self::{
    identifier::{YggdrasilIdentifier, YggdrasilNamepath},
    modifiers::YggdrasilAnnotations,
};
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
pub enum GrammarAtomic {
    Optimized,
    Atomic,
    Combined,
}

impl GrammarAtomic {
    pub fn optimize(&mut self) {
        *self = GrammarAtomic::Optimized
    }
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
    /// Redirect to other rules when parsing is successful
    pub redirect: Option<YggdrasilIdentifier>,
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
    /// Ignore this node in ast mode.
    ///
    /// ## Examples
    /// ```ygg
    /// #atomic(true)
    /// atomic class Rule { }
    /// ```
    pub atomic: GrammarAtomic,
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
    pub body: Option<YggdrasilExpression>,
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

impl Default for GrammarRule {
    fn default() -> Self {
        Self {
            name: Default::default(),
            r#type: String::new(),
            document: String::new(),
            public: false,
            derives: RuleDerive::default(),
            atomic: GrammarAtomic::Atomic,
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            ignored: false,
            kind: GrammarRuleKind::Class,
            body: None,
            range: Default::default(),
            redirect: None,
        }
    }
}

impl GrammarRule {
    pub fn create_class(name: YggdrasilIdentifier, range: Range<usize>) -> Self {
        Self { kind: GrammarRuleKind::Class, name, range, ..Default::default() }
    }
    pub fn create_union(name: YggdrasilIdentifier, range: Range<usize>) -> Self {
        Self { kind: GrammarRuleKind::Union, name, range, ..Default::default() }
    }
    pub fn with_annotation(mut self, extra: &YggdrasilAnnotations) -> Self {
        self.atomic = extra.get_atomic();
        self.ignored = extra.get_ignored();
        self
    }
    pub fn with_expression(mut self, extra: Option<YggdrasilExpression>) -> Self {
        let empty = match &extra {
            Some(s) => match &s.kind {
                ExpressionKind::Choice(v) => v.branches.is_empty(),
                ExpressionKind::Concat(v) => v.sequence.is_empty(),
                _ => false,
            },
            None => true,
        };
        self.body = extra;
        if empty {
            self.body = None
        }
        self
    }
}
