mod classes;
pub mod derive;

mod fields;
mod identifier;
mod unions;

mod annotations;

pub use self::{
    annotations::{YggdrasilAnnotations, YggdrasilMacroArgument, YggdrasilMacroCall, YggdrasilModifiers},
    classes::YggdrasilVariants,
    derive::RuleDerive,
    fields::{counter::FieldCounter, FieldKind, YggdrasilField},
    identifier::{YggdrasilIdentifier, YggdrasilNamepath},
    unions::YggdrasilEnumerates,
};
use crate::{
    data::RuleReference,
    nodes::{ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
};
use convert_case::{Case, Casing};
pub use num::BigInt;
use num::Zero;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    ops::{BitAndAssign, BitOrAssign, BitXorAssign, Range},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionRule {}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Don't capture any objects in rule.
    ///
    /// ## Examples
    /// ```ygg
    /// #tag(true)
    /// class Rule { Tagged }
    /// #tag(false)
    /// class Rule ^ { Untagged }
    /// ```
    pub auto_tag: bool,
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
    /// #hide(true)
    /// hidden class Rule { }
    /// ```
    pub hide: bool,

    /// Ignore this node in ast mode.
    ///
    /// ## Examples
    /// ```ygg
    /// #ignore(true)
    /// ignore class Rule { }
    /// ```
    pub ignored: bool,
    ///
    pub body: GrammarBody,
    /// position of all parts
    pub range: Range<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarBody {
    Empty {},
    Class { term: YggdrasilExpression },
    Union { branches: Vec<YggdrasilExpression> },
    Climb { priority: Vec<YggdrasilExpression> },
    TokenSet { rules: Vec<YggdrasilIdentifier> },
}

impl GrammarBody {
    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut YggdrasilExpression),
    {
        match self {
            GrammarBody::Empty { .. } => {}
            GrammarBody::Class { term } => f(term),
            GrammarBody::Union { branches } => branches.iter_mut().for_each(f),
            GrammarBody::Climb { priority } => priority.iter_mut().for_each(f),
            GrammarBody::TokenSet { .. } => {}
        }
    }
}

impl Default for GrammarBody {
    fn default() -> Self {
        Self::Empty {}
    }
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

impl Default for GrammarRule {
    fn default() -> Self {
        Self {
            name: Default::default(),
            document: String::new(),
            derives: RuleDerive::default(),
            atomic: GrammarAtomic::Atomic,
            auto_inline: false,
            auto_tag: false,
            entry: false,
            hide: false,
            ignored: false,
            body: Default::default(),
            range: Default::default(),
            redirect: None,
        }
    }
}

impl GrammarRule {
    pub fn is_class(&self) -> bool {
        matches!(self.body, GrammarBody::Class { .. })
    }
    pub fn is_union(&self) -> bool {
        matches!(self.body, GrammarBody::Union { .. })
    }
    pub fn node_name(&self) -> String {
        format!("{}Node", self.name.text).to_case(Case::Pascal)
    }
    pub fn parser_name(&self) -> String {
        format!("parse_{}", self.name.text).to_case(Case::Snake)
    }
}

impl GrammarRule {
    pub fn create_class(name: YggdrasilIdentifier, body: Option<YggdrasilExpression>, range: Range<usize>) -> Self {
        match body {
            Some(s) => Self { name, range, body: GrammarBody::Class { term: s }, ..Default::default() },
            None => Self { name, range, body: GrammarBody::Empty {}, ..Default::default() },
        }
    }
    pub fn create_union(name: YggdrasilIdentifier, body: Vec<YggdrasilExpression>, range: Range<usize>) -> Self {
        if body.is_empty() {
            Self { name, range, body: GrammarBody::Empty {}, ..Default::default() }
        }
        else {
            Self { name, range, body: GrammarBody::Union { branches: body }, ..Default::default() }
        }
    }
    pub fn with_annotation(mut self, extra: &YggdrasilAnnotations) -> Self {
        self.atomic = extra.get_atomic();
        self.ignored = extra.get_ignored();
        self.hide = extra.get_keep();
        self.entry = extra.get_entry();
        self
    }
    /// Whether to automatically mark all tags in the rule
    ///
    /// To ensure the highest priority, it needs to be called after with_annotation
    pub fn with_auto_tag(self, on: bool) -> Self {
        Self { auto_tag: on, ..self }
    }
}
