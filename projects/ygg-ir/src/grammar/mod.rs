use crate::{
    nodes::{ExpressionKind, ExpressionNode},
    rule::{FunctionRule, GrammarRule},
    traits::CodeOptimizer,
};
use diagnostic_quick::{QError, QResult, Validation};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    mem::take,
};
use url::Url;

mod auto_tag;
mod dead_code;
mod emit_function;
mod fuse_charset;
mod fuse_rule;
mod inlining;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: String,
    pub extensions: Vec<String>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub ignores: HashSet<String>,
    pub rules: IndexMap<String, GrammarRule>,
    pub functions: IndexMap<String, FunctionRule>,
    pub range_type: String,
}

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            name: "".to_string(),
            extensions: vec![],
            imports: Default::default(),
            exports: vec![],
            ignores: Default::default(),
            rules: Default::default(),
            functions: Default::default(),
            range_type: "usize".to_string(),
        }
    }
}

impl GrammarInfo {
    pub fn ignored_rules(&self) -> Vec<GrammarRule> {
        todo!()
    }
}
