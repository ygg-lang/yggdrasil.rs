use lsp_types::{Diagnostic, DocumentSymbolResponse, Range, Url};
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::{
    hash_map::{Keys, Values},
    HashMap,
};
use tree_sitter_cli::generate::grammars::InputGrammar;

use crate::{ast::StringRanged, manager::HintItems, Result};

use super::*;

pub use self::expression::*;

mod expression;
mod from_ast;
mod hints;
mod input_grammar;
mod optimize;

pub type Map<K, V> = std::collections::HashMap<K, V>;
// pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[derive(Clone, Debug)]
pub struct YGGRule {
    ///
    name: String,
    /// position of the rule
    name_position: Range,
    /// position of all parts
    range: Range,
    ///
    structure_name: Option<String>,
    ///
    force_inline: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// name <- expr
    /// ^expr
    /// ```
    already_inline: bool,
    eliminate_unmarked: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// "string"
    /// /regex/
    /// [0-9a-z]
    /// 012345
    /// ```
    eliminate_unnamed: bool,
    ///
    expression: RefinedExpression,
}

#[derive(Clone, Debug)]
pub struct MetaInfo {
    name: String,
    exts: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct GrammarState {
    url: Url,
    is_grammar: bool,
    name: String,
    name_position: Range,
    extensions: Vec<(String, Range)>,
    ignores: Vec<(String, Range)>,
    pub(self) rule_map: Map<String, YGGRule>,
}

impl GrammarState {
    #[inline]
    pub fn get(&self, rule: &str) -> Option<&YGGRule> {
        self.rule_map.get(rule)
    }
    pub fn get_inline(&self, rule: &str) -> Option<&YGGRule> {
        match self.rule_map.get(rule) {
            Some(s) => Some(s),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn get_mut(&mut self, rule: &str) -> Option<&mut YGGRule> {
        self.rule_map.get_mut(rule)
    }
    #[inline]
    pub fn get_inline_mut(&mut self, rule: &str) -> Option<&mut YGGRule> {
        match self.rule_map.get(rule) {
            Some(_) => self.rule_map.get_mut(rule),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get_mut(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn insert(&mut self, name: String, rule: YGGRule) -> Option<YGGRule> {
        self.rule_map.insert(name, rule)
    }
    #[inline]
    pub fn keys(&self) -> Keys<String, YGGRule> {
        self.rule_map.keys()
    }
    #[inline]
    pub fn rules(&self) -> Values<String, YGGRule> {
        self.rule_map.values()
    }
    #[inline]
    pub fn named_rules(&self) -> Vec<YGGRule> {
        self.rule_map.values().cloned().filter(|r| !r.force_inline).collect()
    }
}

impl YGGRule {
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
