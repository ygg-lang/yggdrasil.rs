use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    ops::{BitAndAssign, BitOrAssign, BitXorAssign, MulAssign, Range},
};

use convert_case::{Case, Casing};
pub use num::BigInt;
use num::Zero;
use yggdrasil_parser::bootstrap::IdentifierNode;

use crate::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
};

pub use self::{
    annotations::{YggdrasilAnnotations, YggdrasilMacroArgument, YggdrasilMacroCall, YggdrasilModifiers},
    atomic::GrammarAtomic,
    classes::YggdrasilVariants,
    derive::RuleDerive,
    fields::{counter::FieldCounter, FieldKind, YggdrasilField},
    identifier::{YggdrasilIdentifier, YggdrasilNamepath},
    unions::YggdrasilEnumerates,
};

mod classes;
pub mod derive;

mod annotations;
mod atomic;
mod fields;
mod identifier;
mod unions;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionRule {}

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
    pub captures: GrammarCaptures,
    ///
    pub body: GrammarBody,
    /// position of all parts
    pub range: Range<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarCaptures {
    /// Don't capture any objects in rule.
    ///
    /// ## Examples
    /// ```ygg
    /// #text(true)
    /// text Rule { Tagged }
    /// ```
    pub text: bool,
    /// Don't capture any objects in rule.
    ///
    /// ## Examples
    /// ```ygg
    /// #tag(true)
    /// class Rule { Tagged }
    /// #tag(false)
    /// class Rule ^ { Untagged }
    /// ```
    pub auto: bool,
    /// Range size
    pub range: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarBody {
    Empty {},
    Class { term: YggdrasilExpression },
    Union { branches: Vec<(Option<YggdrasilIdentifier>, YggdrasilExpression)> },
    Climb { priority: Vec<(Option<YggdrasilIdentifier>, YggdrasilExpression)> },
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
            GrammarBody::Union { branches } => branches.iter_mut().for_each(|v| f(&mut v.1)),
            GrammarBody::Climb { priority } => priority.iter_mut().for_each(|v| f(&mut v.1)),
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
            redirect: None,
            document: String::new(),
            derives: RuleDerive::default(),
            atomic: GrammarAtomic::Atomic,
            auto_inline: false,
            entry: false,
            hide: false,
            ignored: false,
            captures: Default::default(),
            body: Default::default(),
            range: Default::default(),
        }
    }
}

impl Default for GrammarCaptures {
    fn default() -> Self {
        Self { range: "usize".to_string(), text: false, auto: false }
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
    pub fn with_annotation(mut self, extra: &YggdrasilAnnotations) -> Self {
        self.atomic = extra.get_atomic();
        self.ignored = extra.get_ignored();
        self.hide = extra.get_keep();
        self.entry = extra.get_entry();
        if let Some(s) = extra.get_auto_capture() {
            self.captures.auto = s
        };
        if let Some(s) = extra.get_text_capture() {
            self.captures.text = s
        }
        self
    }
}
