use std::{fmt::Debug, mem::swap};

use convert_case::{Case, Casing};
use lsp_types::{Range, Url};

use yggdrasil_bootstrap::{
    ast::{AssignStatement, Program, Statement, StringLiteral, Symbol},
    shared::records::LineBreaks,
    Result,
};

use crate::manager::HintItems;

use self::remap::{Keys, Map, Values};
pub use self::{
    from_ast::{FilePosition, Translator},
    node::*,
};

mod from_ast;
mod hints;
mod node;

// used for ide hint
#[cfg(debug_assertions)]
mod remap {
    pub use std::collections::hash_map::{Keys, Values};

    pub type Map<K, V> = std::collections::HashMap<K, V>;
}

#[cfg(not(debug_assertions))]
mod remap {
    pub use indexmap::map::{Keys, Values};

    pub type Map<K, V> = indexmap::IndexMap<K, V>;
}

#[derive(Clone, Debug)]
pub struct GrammarState {
    pub(crate) url: Url,
    pub(crate) text: String,
    pub(crate) is_grammar: bool,
    pub(crate) name: Symbol,
    pub(crate) extensions: Vec<StringLiteral>,
    pub(crate) ignores: Vec<Symbol>,
    pub(crate) rule_map: Map<String, Rule>,
}

#[derive(Clone)]
pub struct Rule {
    ///
    pub(crate) name: Symbol,
    ///
    pub(crate) ty: Symbol,
    ///
    pub(crate) doc: String,
    ///
    pub(crate) force_inline: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// name <- expr
    /// ^expr
    /// ```
    pub(crate) already_inline: bool,
    pub(crate) eliminate_unmarked: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// "string"
    /// /regex/
    /// [0-9a-z]
    /// 012345
    /// ```
    pub(crate) eliminate_unnamed: bool,
    ///
    pub(crate) expression: ExpressionNode,
    /// position of all parts
    pub(crate) range: (usize, usize),
}

impl GrammarState {
    #[inline]
    pub fn get(&self, rule: &str) -> Option<&Rule> {
        self.rule_map.get(rule)
    }
    pub fn get_inline(&self, rule: &str) -> Option<&Rule> {
        match self.rule_map.get(rule) {
            Some(s) => Some(s),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn get_mut(&mut self, rule: &str) -> Option<&mut Rule> {
        self.rule_map.get_mut(rule)
    }
    #[inline]
    pub fn get_inline_mut(&mut self, rule: &str) -> Option<&mut Rule> {
        match self.rule_map.get(rule) {
            Some(_) => self.rule_map.get_mut(rule),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get_mut(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn insert(&mut self, name: String, rule: Rule) -> Option<Rule> {
        self.rule_map.insert(name, rule)
    }
    #[inline]
    pub fn keys(&self) -> Keys<String, Rule> {
        self.rule_map.keys()
    }
    #[inline]
    pub fn rules(&self) -> Values<String, Rule> {
        self.rule_map.values()
    }
    #[inline]
    pub fn named_rules(&self) -> Vec<Rule> {
        self.rule_map.values().cloned().filter(|r| !r.force_inline).collect()
    }
}

impl Rule {
    pub fn build_structure(&self) -> String {
        unimplemented!()
    }
    pub fn build_parse(&self) -> String {
        unimplemented!()
    }
    pub fn build_error(&self) -> String {
        unimplemented!()
    }
}
