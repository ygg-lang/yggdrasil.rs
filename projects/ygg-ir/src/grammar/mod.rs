use std::collections::{BTreeMap, HashSet};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;

use yggdrasil_error::YggdrasilError;

use crate::{traits::CodeOptimizer, *};

pub mod auto_tag;
pub mod dead_code;
pub mod emit_function;
pub mod fuse_charset;
pub mod fuse_rule;
pub mod inlining;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: String,
    pub extensions: Vec<String>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub rules: IndexMap<String, GrammarRule>,
    pub functions: IndexMap<String, FunctionRule>,
    pub rule_prefix: String,
    pub rule_suffix: String,
}

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            name: "".to_string(),
            extensions: vec![],
            imports: Default::default(),
            exports: vec![],
            rules: Default::default(),
            functions: Default::default(),
            rule_prefix: "".to_string(),
            rule_suffix: "Node".to_string(),
        }
    }
}

impl GrammarInfo {
    pub fn ignored_rules(&self) -> Vec<GrammarRule> {
        todo!()
    }
}
