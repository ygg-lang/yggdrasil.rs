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
    ignores: Vec<String>,
    map: Map<String, YGGRule>,
}

impl GrammarState {
    #[inline]
    pub fn get(&self, rule: &str) -> Option<&YGGRule> {
        self.map.get(rule)
    }
    #[inline]
    pub fn insert(&mut self, name: String, rule: YGGRule) -> Option<YGGRule> {
        self.map.insert(name, rule)
    }
    #[inline]
    pub fn keys(&self) -> Keys<String, YGGRule> {
        self.map.keys()
    }
    #[inline]
    pub fn rules(&self) -> Values<String, YGGRule> {
        self.map.values()
    }
    #[inline]
    pub fn named_rules(&self) -> Vec<YGGRule> {
        self.map.values().cloned().filter(|r| !r.force_inline).collect()
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
