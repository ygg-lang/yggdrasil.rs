pub use self::expression::*;
use super::*;
use lsp_types::{Diagnostic, DocumentSymbolResponse, Range, Url};
use tree_sitter_cli::generate::grammars::InputGrammar;
use rkyv::{Archive,Serialize,Deserialize};
mod expression;
mod from_ast;
mod input_grammar;

pub type Map<K, V> = std::collections::HashMap<K, V>;
// pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[derive(Clone, Debug)]
pub struct YGGRule {
    ///
    name: String,
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
    ///
    range: Range,
}

#[derive(Clone, Debug)]
pub struct MetaInfo {
    name: String,
    exts: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct GrammarState {
    url: Option<Url>,
    name: String,
    map: Map<String, YGGRule>,
    ignores: Vec<String>,
}

impl GrammarState {
    pub fn optimize(&mut self) {
        self.merge_regex();
        self.inline()
    }
    fn inline(&mut self) {}
    fn merge_regex(&mut self) {}
    pub fn named_rules(&self) -> Vec<YGGRule> {
        self.map.values().cloned().filter(|r| !r.force_inline).collect()
    }
    pub fn set_url(&mut self, url: Url) {
        self.url = Some(url)
    }
    pub fn show_document_symbol(&self) -> DocumentSymbolResponse {
        DocumentSymbolResponse::Nested(vec![])
    }
}

impl YGGRule {
    fn inline(&mut self, map: &GrammarState) {}
    fn merge_regex(&mut self) {}
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
