pub use self::derive::RuleDerive;
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

mod classes;
pub mod derive;

mod fields;
mod identifier;

mod modifiers;

pub use self::{
    fields::{counter::FieldCounter, FieldKind, YggdrasilField},
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
    /// def RuleName { }
    /// ```
    pub name: YggdrasilIdentifier,
    /// Redirect to other rules when parsing is successful
    ///
    /// ## Examples
    /// ```ygg
    /// def RuleName -> Redirect { }
    /// ```
    pub redirect: Option<YggdrasilIdentifier>,
    /// Kind of this rule
    pub kind: GrammarRuleKind,
    /// Document of this rule
    ///
    /// ## Examples
    /// ```ygg
    /// /// this is documents
    /// class Rule { }
    /// ```
    pub document: String,
    ///
    /// ## Examples
    /// ```ygg
    /// #derive(Copy, Clone)
    /// #derive(Encode, feature: a)
    /// class Rule { }
    /// ```
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
    /// inline class Rule { }
    /// class _Rule { }
    /// ```
    pub auto_inline: bool,
    /// The entry of the ast mode, the name of the ast_mode to be exported
    ///
    /// ## Examples
    /// ```ygg
    /// #entry(true)
    /// entry class Rule { }
    /// ```
    pub entry: bool,
    /// Keep this rule even if it is not used.
    ///
    /// ## Examples
    /// ```ygg
    /// #keep(true)
    /// keep class Rule { }
    /// ```
    pub keep: bool,
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

// Class rule
pub struct YggdrasilVariant {
    pub document: String,
    pub name: YggdrasilIdentifier,
    pub fields: BTreeMap<String, YggdrasilField>,
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
            document: String::new(),
            derives: RuleDerive::default(),
            atomic: GrammarAtomic::Atomic,
            auto_inline: false,
            entry: false,
            keep: false,
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
        self.keep = extra.get_keep();
        self.entry = extra.get_entry();
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
