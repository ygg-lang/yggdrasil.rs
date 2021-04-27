pub use self::expression::*;
use super::*;
use lsp_types::{Diagnostic, DocumentSymbolResponse, Range, Url};
use rkyv::{Archive, Deserialize, Serialize};
use tree_sitter_cli::generate::grammars::InputGrammar;
mod expression;
mod from_ast;
mod input_grammar;
mod optimize;
use crate::{manager::HintItems, Result};

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
    url: Url,
    name: String,
    map: Map<String, YGGRule>,
    ignores: Vec<String>,
}

impl GrammarState {
    pub fn named_rules(&self) -> Vec<YGGRule> {
        self.map.values().cloned().filter(|r| !r.force_inline).collect()
    }
    pub fn show_document_symbol(&self) -> DocumentSymbolResponse {
        DocumentSymbolResponse::Nested(vec![])
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
