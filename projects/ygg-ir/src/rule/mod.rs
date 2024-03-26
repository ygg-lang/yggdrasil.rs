use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::{BitAndAssign, BitOrAssign, MulAssign, Range},
};

use convert_case::{Case, Casing};
use indexmap::IndexMap;
pub use num::{BigInt, Num};

use crate::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
};

pub use self::{
    annotations::{YggdrasilMacroArgument, YggdrasilMacroCall, YggdrasilModifiers},
    atomic::GrammarAtomic,
    classes::FieldMap,
    derive::RuleDerive,
    fields::{counter::YggdrasilCounter, YggdrasilField},
    identifier::{YggdrasilIdentifier, YggdrasilNamepath},
    unions::{YggdrasilBranch, YggdrasilVariant},
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarRule {
    /// Determine the name of the parser
    ///
    /// If there is no redirect rule, it is also the name of the type.
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

    pub attributes: GrammarRuleAttributes,
    ///
    pub body: GrammarBody,
    /// position of all parts
    pub range: Range<usize>,
}

impl Hash for GrammarRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.text.hash(state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarRuleAttributes {
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
    /// This rule should not generate nodes
    ///
    /// ## Examples
    /// ```ygg
    /// #inline(true)
    /// inline class Rule { }
    /// ```
    pub inline: bool,
    /// The entry of the ast mode, the name of the ast_mode to be exported
    ///
    /// ## Examples
    /// ```ygg
    /// #entry(true)
    /// entry class Rule { }
    /// ```
    pub viewer: GrammarViewer,
    /// Don't capture any objects in rule.
    ///
    /// ## Examples
    /// ```ygg
    /// #tag(true)
    /// class Rule { Tagged }
    /// #tag(false)
    /// class Rule ^ { Untagged }
    /// ```
    pub captures: bool,
}

impl GrammarRuleAttributes {
    pub fn with_hidden(self, hide: bool) -> Self {
        Self { viewer: GrammarViewer { hidden: hide, ..self.viewer }, ..self }
    }
}

impl Default for GrammarRuleAttributes {
    fn default() -> Self {
        Self {
            document: String::new(),
            derives: RuleDerive::default(),
            atomic: GrammarAtomic::default(),
            inline: false,
            viewer: GrammarViewer::default(),
            captures: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarViewer {
    /// Hide this node in normal show
    ///
    /// ## Examples
    /// ```ygg
    /// #hidden(true)
    /// hide class Rule { }
    /// ```
    pub hidden: bool,
    pub styles: Vec<String>,
    pub railway: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarBody {
    // Empty {},
    Class {
        term: YggdrasilExpression,
    },
    Union {
        /// The union branches
        branches: Vec<YggdrasilBranch>,
        /// The union mapping
        refined: IndexMap<String, String>,
    },
    Climb {
        priority: Vec<YggdrasilBranch>,
    },
    // TokenSet { rules: Vec<YggdrasilIdentifier> },
}

impl GrammarBody {
    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut YggdrasilExpression),
    {
        match self {
            Self::Class { term } => f(term),
            Self::Union { .. } => {}
            Self::Climb { .. } => {}
        }
    }
}

impl Default for GrammarBody {
    fn default() -> Self {
        Self::Union { branches: vec![], refined: Default::default() }
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
        Self { name: Default::default(), redirect: None, attributes: Default::default(), body: Default::default(), range: Default::default() }
    }
}

impl Default for GrammarViewer {
    fn default() -> Self {
        Self { hidden: false, styles: vec![], railway: true }
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
        let id = match &self.redirect {
            Some(s) => s.text.as_str(),
            None => self.name.text.as_str(),
        };
        format!("{}Node", id).to_case(Case::Pascal)
    }
    pub fn parser_name(&self) -> String {
        format!("parse_{}", self.name.text).to_case(Case::Snake)
    }
    pub fn merge_annotations(self, annotations: GrammarRuleAttributes) -> Self {
        Self { attributes: annotations, ..self }
    }
}
