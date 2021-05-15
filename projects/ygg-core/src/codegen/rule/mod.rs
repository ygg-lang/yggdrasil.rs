pub use self::node::*;
use self::remap::{Keys, Map, Values};
use crate::{
    ast::{Identifier, StringLiteral},
    Result,
};
use lsp_types::{Range, Url};
use std::fmt::Debug;

mod build;
mod from_ast;
mod hints;
mod node;
mod optimize;

// used for ide hint
#[cfg(debug_assertions)]
mod remap {
    pub type Map<K, V> = std::collections::HashMap<K, V>;
    pub use std::collections::hash_map::{Keys, Values};
}
#[cfg(not(debug_assertions))]
mod remap {
    pub type Map<K, V> = indexmap::IndexMap<K, V>;
    pub use indexmap::map::{Keys, Values};
}

#[derive(Clone, Debug)]
pub struct GrammarState {
    url: Url,
    is_grammar: bool,
    name: Identifier,
    extensions: Vec<StringLiteral>,
    ignores: Vec<Identifier>,
    rule_map: Map<String, YGGRule>,
}

#[derive(Clone)]
pub struct YGGRule {
    ///
    name: Identifier,
    ///
    ty: Identifier,
    ///
    doc: String,
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
    expression: ExpressionNode,
    /// position of all parts
    range: Range,
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
